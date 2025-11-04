
terraform {
  backend "local" {
    path = "terraform.tfstate"
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
