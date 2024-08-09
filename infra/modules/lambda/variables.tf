variable "function_name" {
  description = "AWS Lambda function name"
}

variable "route_key" {
  description = "Route key to be used by API Gateway"
}

variable "gateway_id" {
  description = "API Gateway ID"
}

variable "gateway_execution_arn" {
  default = "API Gateway execution arn"
}
