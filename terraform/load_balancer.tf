resource "aws_lb" "test-balancer" {
  name               = "test-balancer-19"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.sg-test.id]
  subnets            = [aws_subnet.public-sn1.id, aws_subnet.public-sn2.id]

  enable_deletion_protection = false

  # access_logs {
  #     bucket = aws_s3_bucket.load_balancer_log.id
  #     prefix = "test-prefix"
  #     enabled = true
  # }

  tags = {
    Name = "test-balancer"
  }
}

resource "aws_lb_target_group" "test-balancer-target-group" {
    name     = "test-balancer-target-group"
    port     = 8080
    protocol = "HTTP"
    vpc_id   = aws_vpc.vpc.id
    target_type = "ip"
    
    
    health_check {
        path                = "/"
        protocol            = "HTTP"
        port                = "traffic-port"
        healthy_threshold   = 5
        unhealthy_threshold = 2
        timeout             = 3
        interval            = 30
    }
    
    tags = {
        Name = "test-balancer-target-group"
    }
}

resource "aws_lb_listener" "test_balancer_listener" {
    load_balancer_arn = aws_lb.test-balancer.arn
    port              = "8080"
    protocol          = "HTTP"

    default_action {
        type             = "forward"
        target_group_arn = aws_lb_target_group.test-balancer-target-group.arn
    }
  
}
