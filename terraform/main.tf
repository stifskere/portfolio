
terraform {
  backend "s3" {
    bucket = "personal"
    key = "terraform.tfstate"
    region = "eu-west-1"

    endpoints = {
      s3 = "https://s3-api.memw.es"
    }

//    skip_credentials_validation = true
    skip_requesting_account_id = true
    skip_metadata_api_check = true
    skip_region_validation = true
    use_path_style = true

    workspace_key_prefix = ""
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
