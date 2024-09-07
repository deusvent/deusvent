variable "certificate-arn" {
  description = "ARN of a certificate to be attached to custom domain"
}

variable "jwt_auth_secret" {
  description = "Secret which is used for creting JWT tokens for authentication"
}
