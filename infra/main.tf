provider "aws" {
  region = "us-east-1"
  default_tags {
    tags = {
      owner = "deusvent.com"
    }
  }
}

module "www" {
  source          = "./www"
  certificate-arn = module.domain.certificate_arn
}

module "domain" {
  source                         = "./domain"
  www-destination-name           = module.www.cloudfront_domain
  www-destination-hosted_zone_id = module.www.cloudfron_zone_id
  api-destination-name           = module.api.api_gateway_domain
  api-destination-hosted_zone_id = module.api.api_gateway_zone_id
}

module "api" {
  source          = "./api"
  certificate-arn = module.domain.certificate_arn
}

module "storage" {
  source     = "./storage"
  table-name = "game_data"
}

# Following setup is for CI and testing purposes

module "storage_test" {
  source     = "./storage"
  table-name = "game_data_test"
}

module "user_ci" {
  source = "./user"
  name   = "ci"
  iam_policies = [
    module.storage_test.iam_reader,
    module.storage_test.iam_writer
  ]
}

output "user_ci_access_key" {
  value = module.user_ci.access_key_id
}

output "user_ci_access_secret" {
  value     = module.user_ci.access_key_secret
  sensitive = true
}
