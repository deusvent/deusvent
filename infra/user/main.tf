resource "aws_iam_user" "user" {
  name = var.name
}

resource "aws_iam_user_policy_attachment" "policy_attachments" {
  for_each   = toset(var.iam_policies)
  user       = aws_iam_user.user.name
  policy_arn = each.value
}

resource "aws_iam_access_key" "access_key" {
  user = aws_iam_user.user.name
}

output "access_key_id" {
  value = aws_iam_access_key.access_key.id
}

output "access_key_secret" {
  value     = aws_iam_access_key.access_key.secret
  sensitive = true
}
