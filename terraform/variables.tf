
variable "cluster" {
  description = "Target cluster connection information."
  type = object({
    // The kubeconfig file path
    kubeconfig = string
  })
  sensitive = true
}

variable "cloudflare" {
  description = "Cloudflare credentials and account information."
  type = object({
    // The cloudflare api token
    api_token = string

    // Where the service will end up (url)
    target_domain = string
    // The cluster target address
    target_address = string
  })
  sensitive = true
}
