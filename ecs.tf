resource "aws_ecs_cluster" "ecs" {
  name = "test_cluster"
}

resource "aws_ecs_service" "test-service" {
  name                   = "test-service"
  cluster                = aws_ecs_cluster.ecs.arn
  launch_type            = "FARGATE"
  enable_execute_command = true

  deployment_maximum_percent         = 200
  deployment_minimum_healthy_percent = 100
  desired_count                      = 1
  task_definition                    = aws_ecs_task_definition.td.arn

  network_configuration {
    assign_public_ip = true
    subnets          = [aws_subnet.public-sn1.id, aws_subnet.public-sn2.id]
    security_groups  = [aws_security_group.sg-test.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.test-balancer-target-group.arn
    container_name   = "test-container"
    container_port   = 8080
  }
}

resource "aws_ecs_task_definition" "td" {
  container_definitions = jsonencode([
    {
      name      = "test-container"
      image     = "262318881725.dkr.ecr.ap-south-1.amazonaws.com/test_repo"
      cpu       = 256
      memory    = 512
      essential = true
      portMappings = [
        {
          containerPort = 8080
          hostPort      = 8080
        }
      ]
    }
  ])
  family                   = "test-task"
  requires_compatibilities = ["FARGATE"]
  cpu                      = 256
  memory                   = 512
  network_mode = "awsvpc"
  task_role_arn            = "arn:aws:iam::262318881725:role/ecsTaskExecutionRole"
  execution_role_arn       = "arn:aws:iam::262318881725:role/ecsTaskExecutionRole"

}
