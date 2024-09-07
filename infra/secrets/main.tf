// For easier development all the secrets are managed by Terraform and then forwarded as env variables to lambdas
// Before going to production all the keys should be re-created and migrated to AWS Secret Manager instead

resource "random_password" "jwt_auth_secret" {
  length = 64
}

output "jwt_auth_secret" {
  value = random_password.jwt_auth_secret.result
}
