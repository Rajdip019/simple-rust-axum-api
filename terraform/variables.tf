variable "region" {
  type        = string
  description = "The region in which the resources will be created"
  default     = "ap-south-1"
}

variable "network_mode" {
  type        = string
  description = "The network mode of the container"
  default     = "awsvpc"
}

variable "operating_system_family" {
  type        = string
  description = "The operating system family of the container"
  default     = "LINUX"
}

variable "cpu_architecture" {
  type        = string
  description = "The CPU architecture of the container"
  default     = "X86_64"
}

variable "ecs_execution_role" {
  type        = string
  description = "The ARN of the ECS execution role"
  default     = "arn:aws:iam::262318881725:role/ecsTaskExecutionRole"
}

variable "port" {
  type        = number
  description = "The port on which the container listens"
  default     = 8080
}

