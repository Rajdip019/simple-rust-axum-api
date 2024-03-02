resource "aws_cloudwatch_log_group" "ecs-logs" {
  name              = "/ecs/test-container"
  tags = {
    Name = "ecs-logs"
  }
}
