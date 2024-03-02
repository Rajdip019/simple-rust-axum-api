resource "aws_cloudwatch_log_group" "ecs-logs" {
  name              = "/ecs/test_cluster"
  retention_in_days = 7
  tags = {
    Name = "ecs-logs"
  }
}

resource "aws_kms_key" "ecs-logs-key" {
  description             = "KMS key for encrypting ECS logs"
  deletion_window_in_days = 7
}
