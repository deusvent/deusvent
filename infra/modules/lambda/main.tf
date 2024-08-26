locals {
  prefix = "api" # Add prefix for all lambdas to allow other types of lambdas in the future
}

data "aws_iam_policy_document" "minimum_access" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_policy" "lambda_logging" {
  name = "lambda-${local.prefix}-${var.function_name}-logging"
  path = "/"
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents",
        ],
        Resource = ["${aws_cloudwatch_log_group.log_group.arn}:*"],
      },
    ],
  })
}

resource "aws_iam_role" "access" {
  name               = "lambda-${local.prefix}-${var.function_name}-access"
  assume_role_policy = data.aws_iam_policy_document.minimum_access.json
}

resource "aws_iam_role_policy_attachment" "lambda_logs" {
  role       = aws_iam_role.access.name
  policy_arn = aws_iam_policy.lambda_logging.arn
}

resource "aws_cloudwatch_log_group" "log_group" {
  name              = "/aws/lambda/${local.prefix}/${var.function_name}"
  retention_in_days = 1
}

# HACK: We can't define empty Lambda without a file, so here some empty one
# Actual Lambda will be created and deployed through the Github Actions
data "archive_file" "empty" {
  type        = "zip"
  output_path = "${path.module}/empty-lambda.zip"
  source {
    content  = "bootstrap"
    filename = "bootstrap"
  }
}

resource "aws_lambda_function" "lambda" {
  filename         = data.archive_file.empty.output_path
  function_name    = "${local.prefix}-${var.function_name}"
  handler          = "bootstrap"
  role             = aws_iam_role.access.arn
  runtime          = "provided.al2023"
  source_code_hash = data.archive_file.empty.output_base64sha256
  architectures    = ["arm64"]
  lifecycle {
    ignore_changes = [
      source_code_hash # We deploy new version via CI, so it can be ignored
    ]
  }
}

resource "aws_apigatewayv2_route" "route" {
  api_id    = var.gateway_id
  route_key = var.route_key
  target    = "integrations/${aws_apigatewayv2_integration.lambda.id}"
}

resource "aws_apigatewayv2_integration" "lambda" {
  api_id           = var.gateway_id
  integration_type = "AWS_PROXY"
  integration_uri  = aws_lambda_function.lambda.invoke_arn
}

resource "aws_lambda_permission" "api_gateway_invoke_permission" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${var.gateway_execution_arn}/*"
}

resource "aws_apigatewayv2_route_response" "response" {
  api_id             = var.gateway_id
  route_id           = aws_apigatewayv2_route.route.id
  route_response_key = "$default"
}
