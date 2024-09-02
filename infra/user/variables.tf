variable "iam_policies" {
  type        = list(string)
  description = "Array of IAM policies to attach to the user"
}

variable "name" {
  description = "Name of the user"
}
