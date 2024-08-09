resource "aws_apigatewayv2_api" "api" {
  name                       = "api"
  protocol_type              = "WEBSOCKET"
  route_selection_expression = "$request.body.action"
}

resource "aws_apigatewayv2_stage" "v1" {
  api_id = aws_apigatewayv2_api.api.id
  name   = "v1"
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
  lambda_routes = {
    health        = "health"
    ws-connect    = "$connect"
    ws-disconnect = "$disconnect"
  }
}

module "lambda_routes" {
  source                = "../modules/lambda"
  for_each              = local.lambda_routes
  function_name         = each.key
  route_key             = each.value
  gateway_id            = aws_apigatewayv2_api.api.id
  gateway_execution_arn = aws_apigatewayv2_api.api.execution_arn
}
