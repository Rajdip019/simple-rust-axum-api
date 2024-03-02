resource "aws_ecr_repository" "repo" {
  name = "test_repo"
  tags = {
    Name = "test_repo"
  }
  image_tag_mutability = "MUTABLE"
  image_scanning_configuration {
    scan_on_push = true
  }
}
