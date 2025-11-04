
// ### cloudflare resources ###


data "cloudflare_zone" "portfolio" {
	name = var.cloudflare.target_domain
}

resource "cloudflare_record" "portfolio" {
	zone_id = data.cloudflare_zone.portfolio.id
  name = "@"
  type = "A"
  value = var.cloudflare.target_address
  proxied = true
}


// ### kubernetes manifests ###

// namespace
resource "kubernetes_namespace" "portfolio" {
  metadata {
    name = "portfolio"
  }
}

// container deployment
resource "kubernetes_deployment" "portfolio" {
  metadata {
    name = "portfolio"
    namespace = kubernetes_namespace.portfolio.metadata[0].name
    labels = {
      app = "portfolio"
    }
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "portfolio"
      }
    }

    template {
      metadata {
        labels = {
          app = "portfolio"
        }
      }

      spec {
        container {
          name = "portfolio"
          image = "ghcr.io/stifskere/portfolio:latest"
          port {
            container_port = 8080
          }
        }
      }
    }
  }
}

// access service
resource "kubernetes_service" "portfolio" {
  metadata {
    name = "portfolio"
    namespace = kubernetes_namespace.portfolio.metadata[0].name
  }

  spec {
    type = "ClusterIP"

    selector = {
      app = "portfolio"
    }

    port {
      port = 8080
      target_port = 8080
    }
  }
}

// ssl certificate from configured provider
resource "kubernetes_manifest" "portfolio_certificate" {
  manifest = {
    apiVersion = "cert-manager.io/v1"
    kind = "Certificate"

    metadata = {
      name = "portfolio-certificate"
      namespace = kubernetes_namespace.portfolio.metadata[0].name
    }

    spec = {
      secretName = "portfolio-tls"
      issuerRef = {
        name = "letsencrypt-prod"
        kind = "ClusterIssuer"
      }
      commonName = data.cloudflare_zone.portfolio.name
      dnsNames = [data.cloudflare_zone.portfolio.name]
    }
  }
}

// ingress
resource "kubernetes_manifest" "portfolio_ingress" {
  manifest = {
    apiVersion = "networking.k8s.io/v1"
    kind = "Ingress"

    metadata = {
      name      = "portfolio"
      namespace = kubernetes_namespace.portfolio.metadata[0].name
      annotations = {
        "traefik.ingress.kubernetes.io/router.middlewares" = "default-redirect-https@kubernetescrd"
        "cert-manager.io/cluster-issuer" = "letsencrypt-prod"
      }
    }

    spec = {
      rules = [
        {
          host = data.cloudflare_zone.portfolio.name
          http = {
            paths = [
              {
                path     = "/"
                pathType = "Prefix"
                backend = {
                  service = {
                    name = "portfolio"
                    port = { number = 8080 }
                  }
                }
              }
            ]
          }
        }
      ]

      tls = [
        {
          hosts      = [data.cloudflare_zone.portfolio.name]
          secretName = "portfolio-tls"
        }
      ]
    }
  }
}
