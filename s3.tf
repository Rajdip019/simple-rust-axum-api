resource "aws_s3_bucket" "load_balancer_log" {
  bucket = "lb-test-bucket-19"

  tags = {
    Name = "lb-test-bucket-19"
  }
}