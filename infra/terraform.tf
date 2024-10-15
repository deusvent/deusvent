terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.71.0"
    }
  }
  backend "s3" {
    bucket  = "deusvent.com.terraform"
    key     = "terraform.tfstate"
    region  = "eu-north-1"
    encrypt = true
  }
}
