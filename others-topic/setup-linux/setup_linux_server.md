# 🐧 Linux Server Setup & Docker Optimization

This guide provides a professional procedure to set up a new Linux server (Ubuntu/Debian) and install Docker with a focus on resource optimization.

---

## 🛠️ Step 1: System Preparation
Update the package index and install essential dependencies.

```bash
# Update local package index
sudo apt update

# Install dependencies to allow apt to use a repository over HTTPS
sudo apt install -y apt-transport-https ca-certificates curl software-properties-common
```

> [!NOTE]
> **Why? | Tại sao?**
> - **en**: `apt update` ensures the package index is current. `apt-transport-https` and `ca-certificates` enable secure HTTPS communication with external repositories.
> - **vi**: `apt update` đảm bảo danh sách gói phần mềm luôn mới nhất. `apt-transport-https` và `ca-certificates` cho phép giao tiếp HTTPS an toàn với các kho lưu trữ bên ngoài.

## 🐳 Step 2: Docker Installation

### 1. Add Docker's Official GPG Key
```bash
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
```

### 2. Set up the Repository
```bash
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

> [!NOTE]
> **Why? | Tại sao?**
> - **en**: The GPG key verifies that the Docker packages are authentic. The official repository provides the latest stable updates directly from Docker.
> - **vi**: Khóa GPG xác minh rằng các gói Docker là chính chủ. Kho lưu trữ chính thức cung cấp các bản cập nhật ổn định mới nhất trực tiếp từ Docker.

### 3. Install Docker Engine
```bash
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

## ⚙️ Step 3: Post-Installation & Optimization

### 👤 1. Manage Docker as a Non-root User
To reduce the overhead of using `sudo` for every command and improve security/workflow:
```bash
sudo usermod -aG docker ${USER}
# Reload groups without relogging
newgrp docker
```

> [!TIP]
> **Why? | Tại sao?**
> - **en**: Running Docker as a non-root user improves security by limiting permissions and makes scripts easier to run without constant password prompts.
> - **vi**: Chạy Docker với tư cách người dùng không phải root giúp tăng cường bảo mật bằng cách giới hạn quyền hạn và giúp các script chạy dễ dàng hơn mà không cần nhập mật khẩu liên tục.

### 💾 2. Log Rotation (Resource Optimization)
By default, Docker logs can grow indefinitely, consuming all disk space. Create or edit `/etc/docker/daemon.json` to limit them:

```json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
```
Apply changes: `sudo systemctl restart docker`

> [!IMPORTANT]
> **Why? | Tại sao?**
> - **en**: Log rotation is critical for small servers to prevent the disk from filling up, which would otherwise crash the entire system.
> - **vi**: Xoay vòng nhật ký rất quan trọng đối với các máy chủ nhỏ để ngăn đĩa bị đầy, nếu không sẽ làm toàn bộ hệ thống bị treo.

---

## 🧭 Syntax Explanation | Giải thích Cú pháp

en: 
1. `sudo apt update`: Updates the package list from repositories. `sudo` grants administrative privileges.
2. `apt-transport-https`: Allows the use of repositories accessed via HTTPS.
3. `curl -fsSL ... | sudo gpg --dearmor`: Fetches the Docker GPG key and converts it from ASCII to a binary format for security verification.
4. `lsb_release -cs`: Dynamically detects your Ubuntu distribution version (e.g., focal, jammy).
5. `tee /etc/apt/sources.list.d/docker.list`: Securely writes the repository configuration to a file.
6. `docker-compose-plugin`: Installs the modern Docker Compose V2.

------------------------------------------------------------------------------

vi:
1. `sudo apt update`: Cập nhật danh sách các gói phần mềm từ các kho lưu trữ (repositories). `sudo` cấp quyền quản trị viên.
2. `apt-transport-https`: Cho phép sử dụng các kho lưu trữ được truy cập qua giao thức HTTPS.
3. `curl -fsSL ... | sudo gpg --dearmor`: Tải về mã khóa GPG của Docker và chuyển đổi nó từ định dạng ASCII sang binary để xác thực bảo mật.
4. `lsb_release -cs`: Tự động phát hiện phiên bản phân phối Ubuntu bạn đang dùng (ví dụ: focal, jammy).
5. `tee /etc/apt/sources.list.d/docker.list`: Ghi cấu hình kho lưu trữ vào tệp tin một cách an toàn.
6. `docker-compose-plugin`: Cài đặt Docker Compose V2 phiên bản hiện đại.


## 🧭 Full syntax

```bash
# 1. Update system and install prerequisite packages
sudo apt update
sudo apt install -y apt-transport-https ca-certificates curl software-properties-common

# 2. Add Docker's official GPG key
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# 3. Add the Docker repository to APT sources
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# 4. Install Docker Engine, CLI, and Compose plugin
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# 5. Enable Docker to run without sudo (User Group)
sudo usermod -aG docker ${USER}

# 6. Configure Log Rotation to optimize disk usage
sudo mkdir -p /etc/docker
cat <<EOF | sudo tee /etc/docker/daemon.json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
EOF

# 7. Restart Docker and refresh group membership
sudo systemctl restart docker
newgrp docker
```

<!-- backup -->
```bash
sudo apt update

sudo apt install -y apt-transport-https ca-certificates curl software-properties-common

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt update

sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

sudo apt-get update

sudo apt-get install docker-compose-plugin
```