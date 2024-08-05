resource "aws_route53_zone" "deusvent" {
  name    = "deusvent.com"
  comment = ""
}

resource "aws_route53_record" "email" {
  zone_id = aws_route53_zone.deusvent.zone_id
  name    = "deusvent.com"
  type    = "MX"
  ttl     = 300
  records = [
    "1 ASPMX.L.GOOGLE.COM.",
    "5 ALT1.ASPMX.L.GOOGLE.COM.",
    "5 ALT2.ASPMX.L.GOOGLE.COM.",
    "10 ALT3.ASPMX.L.GOOGLE.COM.",
    "10 ALT4.ASPMX.L.GOOGLE.COM.",
  ]
}

resource "aws_route53_record" "www" {
  zone_id = aws_route53_zone.deusvent.zone_id
  name    = "www.deusvent.com"
  type    = "A"
  alias {
    name                   = var.www-destination-name
    zone_id                = var.www-destination-hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "naked" {
  zone_id = aws_route53_zone.deusvent.zone_id
  name    = "deusvent.com"
  type    = "A"
  alias {
    name                   = aws_route53_record.www.name
    zone_id                = aws_route53_record.www.zone_id
    evaluate_target_health = false
  }
}

resource "aws_acm_certificate" "certificate" {
  domain_name               = "*.deusvent.com"
  validation_method         = "EMAIL"
  subject_alternative_names = ["deusvent.com"]
}

output "certificate_arn" {
  value = aws_acm_certificate.certificate.arn
}
