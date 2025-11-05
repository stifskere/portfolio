
variable "cluster" {
  description = "Target cluster connection information."
  type = object({
    // The kubeconfig file path.
    kubeconfig = string

    // Where the container is deployed.
    deployment_tag = string
  })
  sensitive = true
}

variable "cloudflare" {
  description = "Cloudflare credentials and account information."
  type = object({
    // The cloudflare api token.
    api_token = string

    // The zone where the domain will be.
    target_zone = string

    // Where the service will end up (url)
    // note that these are simple type A records.
    //
    // record_key => record_content
    target_records = map(string)
  })
  sensitive = true
}
