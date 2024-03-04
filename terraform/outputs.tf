output "LOAD_BALANCER_DNS" {
  value = aws_lb.test-balancer.dns_name
}