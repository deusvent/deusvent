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
}

module "api" {
  source = "./api"
  certificate-arn = module.domain.certificate_arn
}
