resource "aws_ecs_cluster" "ecs" {
  name = "test_cluster"
  configuration {

    execute_command_configuration {
      logging    = "OVERRIDE"
      log_configuration {
        cloud_watch_log_group_name     = aws_cloudwatch_log_group.ecs-logs.name
      }
    }
  }
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
      cpu       = 0
      essential = true
      portMappings = [
        {
          name          = "test-container-http"
          containerPort = 8080
          hostPort      = 8080
          protocol      = "tcp"
          appProtocol   = "http"
        }
      ]
      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-create-group"  = "true"
          "awslogs-group"         = aws_cloudwatch_log_group.ecs-logs.name
          "awslogs-region"        = "ap-south-1"
          "awslogs-stream-prefix" = "ecs"
        }
      }
    }
  ])
  family                   = "test-task"
  requires_compatibilities = ["FARGATE"]
  cpu                      = 2048
  memory                   = 4096
  network_mode             = "awsvpc"
  runtime_platform {
    operating_system_family = "LINUX"
    cpu_architecture        = "X86_64"
  }
  task_role_arn      = "arn:aws:iam::262318881725:role/ecsTaskExecutionRole"
  execution_role_arn = "arn:aws:iam::262318881725:role/ecsTaskExecutionRole"
}
