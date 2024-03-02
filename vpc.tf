resource "aws_vpc" "vpc" {
  cidr_block         = "10.0.0.0/16"
  instance_tenancy   = "default"
  enable_dns_support = true
  tags = {
    Name = "test"
  }
}

resource "aws_subnet" "public-sn1" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.1.0/24"
  availability_zone       = "ap-south-1a"
  map_public_ip_on_launch = true
  tags = {
    Name = "public-sn1"
  }
}

resource "aws_subnet" "public-sn2" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.2.0/24"
  availability_zone       = "ap-south-1b"
  map_public_ip_on_launch = true
  tags = {
    Name = "public-sn2"
  }
}

resource "aws_subnet" "public-sn3" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.3.0/24"
  availability_zone       = "ap-south-1c"
  map_public_ip_on_launch = true
  tags = {
    Name = "public-sn3"
  }
}

resource "aws_security_group" "sg-test" {
  name   = "test"
  vpc_id = aws_vpc.vpc.id
  tags = {
    Name = "test"
  }

  ingress {
    description = "https"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "http"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.vpc.id
  tags = {
    Name = "test"
  }
}

resource "aws_route_table" "public-rt" {
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.igw.id
  }

  route {
    ipv6_cidr_block        = "::/0"
    gateway_id             = aws_internet_gateway.igw.id
  }
  tags = {
    Name = "public-rt"
  }
}

resource "aws_route_table_association" "route1" {
  route_table_id = aws_route_table.public-rt.id
  subnet_id      = aws_subnet.public-sn1.id
}

resource "aws_route_table_association" "route2" {
  route_table_id = aws_route_table.public-rt.id
  subnet_id      = aws_subnet.public-sn2.id
}

resource "aws_route_table_association" "route3" {
  route_table_id = aws_route_table.public-rt.id
  subnet_id      = aws_subnet.public-sn3.id
}
