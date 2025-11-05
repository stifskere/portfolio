
terraform {
  backend "s3" {
    skip_requesting_account_id = true
    skip_credentials_validation = true
    skip_metadata_api_check = true
    skip_region_validation = true
    use_path_style = true
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
