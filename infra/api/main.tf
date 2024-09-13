resource "aws_apigatewayv2_api" "api" {
  name                       = "api"
  protocol_type              = "WEBSOCKET"
  route_selection_expression = "$request.body.type"
}

resource "aws_apigatewayv2_stage" "v1" {
  api_id      = aws_apigatewayv2_api.api.id
  name        = "v1"
  auto_deploy = true

  default_route_settings {
    data_trace_enabled = true
    logging_level      = "OFF" // Set to "INFO" to enable verbose logging on all the requests for debugging purposes

    // Rate limits are pure guess for now. If values are missing then it's assumed 0 and everything fails with 409 error
    throttling_burst_limit = 50
    throttling_rate_limit  = 100
  }
}

resource "aws_apigatewayv2_domain_name" "api" {
  domain_name = "api.deusvent.com"
  domain_name_configuration {
    certificate_arn = var.certificate-arn
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }
}

resource "aws_apigatewayv2_api_mapping" "domain_mapping" {
  api_id      = aws_apigatewayv2_api.api.id
  domain_name = aws_apigatewayv2_domain_name.api.id
  stage       = aws_apigatewayv2_stage.v1.id
}

output "api_gateway_domain" {
  value = aws_apigatewayv2_domain_name.api.domain_name_configuration[0].target_domain_name
}

output "api_gateway_zone_id" {
  value = aws_apigatewayv2_domain_name.api.domain_name_configuration[0].hosted_zone_id
}

locals {
  // Configuration for all API lambdas, following keys are supported:
  // - name: name of a lambda, required
  // - route: API Gateway routing key, required
  // - iam_policies: Array of IAM policies to be attached to the lambda
  // - env_variables: Map of environment variables for the lambda
  lambdas = [
    { name = "common-ping", route = "common.ping" },
    { name = "ws-connect", route = "$connect" },
    { name = "ws-disconnect", route = "$disconnect" },
  ]
}

module "lambda_routes" {
  source                = "../modules/lambda"
  gateway_id            = aws_apigatewayv2_api.api.id
  gateway_execution_arn = aws_apigatewayv2_api.api.execution_arn
  for_each              = { for idx, lambda in local.lambdas : lambda.name => lambda }
  function_name         = each.value.name
  route_key             = each.value.route
  iam_policies          = lookup(each.value, "iam_policies", [])
  env_variables         = lookup(each.value, "env_variables", {})
}

// Logs
data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["apigateway.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "cloudwatch" {
  name               = "api_gateway_cloudwatch_global"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

data "aws_iam_policy_document" "cloudwatch_policy" {
  statement {
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
      "logs:DescribeLogStreams"
    ]
    resources = ["arn:aws:logs:*:*:*"]
  }
}

resource "aws_iam_role_policy" "cloudwatch_policy" {
  name   = "api_gateway_cloudwatch_policy"
  role   = aws_iam_role.cloudwatch.id
  policy = data.aws_iam_policy_document.cloudwatch_policy.json
}

resource "aws_api_gateway_account" "api" {
  cloudwatch_role_arn = aws_iam_role.cloudwatch.arn
}
