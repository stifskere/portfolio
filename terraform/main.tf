
terraform {
  backend "s3" {
    endpoint = "https://s3.memw.es"
    bucket = "personal"
    key = "portfolio/terraform.tfstate"
    region = "eu-west-1"

    skip_credentials_validation = true
    skip_metadata_api_check = true
    force_path_style = true
  }

	required_providers {
		kubernetes = {
			source = "hashicorp/kubernetes"
			version = "~> 2.30"
		}

		cloudflare = {
			source = "cloudflare/cloudflare"
			version = "~> 4.0"
		}
	}
}

provider "kubernetes" {
	config_path = var.cluster.kubeconfig
}

provider "cloudflare" {
	api_token = var.cloudflare.api_token
}
