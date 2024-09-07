variable "function_name" {
  description = "AWS Lambda function name"
}

variable "route_key" {
  description = "Route key to be used by API Gateway WebSocket routing"
}

variable "gateway_id" {
  description = "API Gateway ID"
}

variable "gateway_execution_arn" {
  description = "API Gateway execution arn"
}

variable "iam_policies" {
  type        = list(string)
  default     = []
  description = "Array of IAM policies that will be attached to the lambda"
}

variable "env_variables" {
  type        = map(string)
  default     = {}
  description = "Map of environment variables for the lambda"
}
