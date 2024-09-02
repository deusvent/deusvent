resource "aws_dynamodb_table" "game_data" {
  name                        = var.table-name
  billing_mode                = "PAY_PER_REQUEST"
  hash_key                    = "pk"
  range_key                   = "sk"
  deletion_protection_enabled = true

  attribute {
    name = "pk"
    type = "S"
  }

  attribute {
    name = "sk"
    type = "S"
  }
}

resource "aws_iam_policy" "game_data_reader" {
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "dynamodb:GetItem",
          "dynamodb:Query",
          # No Scan as reading should use only Query functionality
        ]
        Effect   = "Allow"
        Resource = aws_dynamodb_table.game_data.arn
      }
    ]
  })
}

resource "aws_iam_policy" "game_data_writer" {
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
          "dynamodb:BatchWriteItem",
        ]
        Effect   = "Allow"
        Resource = aws_dynamodb_table.game_data.arn
      }
    ]
  })
}

output "iam_reader" {
  value = aws_iam_policy.game_data_reader.arn
}

output "iam_writer" {
  value = aws_iam_policy.game_data_writer.arn
}
