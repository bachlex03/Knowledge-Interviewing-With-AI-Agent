```bash
sudo apt update

sudo apt install python3 python3-venv libaugeas0

sudo python3 -m venv /opt/certbot/

sudo /opt/certbot/bin/pip install --upgrade pip

#Install Certbot
sudo /opt/certbot/bin/pip install certbot certbot-nginx

sudo ln -s /opt/certbot/bin/certbot /usr/bin/certbot

sudo certbot certonly --standalone -d example.com -d admin.example.com

sudo cp /etc/letsencrypt/live/example.com/fullchain.pem /path/to/your/destination/
sudo cp /etc/letsencrypt/live/example.com/privkey.pem /path/to/your/destination/
```

## map to docker compose
```yaml
services:
    nginx.webserver:
        image: nginx:stable
        ports:
        - "80:80"
        - "443:443"
        volumes:
        - ./nginx.fe.conf:/etc/nginx/nginx.conf
        - /etc/letsencrypt:/etc/letsencrypt:ro
        depends_on:
        - client.web
        networks:
        - client-network
    
    client.web:
        build:
            context: ./Client
            dockerfile: Dockerfile
        ports:
        - "3000:3000"
        networks:
        - client-network

    admin.web:
        build:
            context: ./Admin
            dockerfile: Dockerfile
        ports:
        - "4000:3000"
        networks:
        - client-network

networks:
  client-network:
    name: client-network
    driver: bridge
```

```nginx config
events {

}

http {
    # HTTP to HTTPS redirection
    server {
        listen 80;
        listen [::]:80;
        server_name ybzone.io.vn admin.ybzone.io.vn;

        # Redirect all HTTP traffic to HTTPS
        return 301 https://$host$request_uri;
    }

    # Main server block for ybzone.io.vn
    server {
        listen 443 ssl; 
        listen [::]:443 ssl;
        server_name ybzone.io.vn;

        ssl_certificate /etc/letsencrypt/live/ybzone.io.vn/fullchain.pem; # Path to your self-signed certificate
        ssl_certificate_key /etc/letsencrypt/live/ybzone.io.vn/privkey.pem; # Path to your self-signed certificate key

        # Optional: Add SSL settings for better security
        # l_protocols TLSv1.2 TLSv1.3;
        # ssl_prefer_server_ciphers on;
        # ssl_ciphers EECDH+AESGCM:EDH+AESGCM:AES256+EECDH:AES256+EDH;
        # ssl_session_cache shared:SSL:10m;
        # ssl_session_timeout 1d;
        # ssl_session_tickets off;

        location / {
            proxy_pass http://client.web:3000;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }

    # Server block for admin.ybzone.io.vn
    server {
        listen 443 ssl; 
        listen [::]:443 ssl;
        server_name admin.ybzone.io.vn;

        ssl_certificate /etc/letsencrypt/live/ybzone.io.vn/fullchain.pem; # Path to your self-signed certificate
        ssl_certificate_key /etc/letsencrypt/live/ybzone.io.vn/privkey.pem; # Path to your self-signed certificate key

        # Optional: Add SSL settings for better security
        # l_protocols TLSv1.2 TLSv1.3;
        # ssl_prefer_server_ciphers on;
        # ssl_ciphers EECDH+AESGCM:EDH+AESGCM:AES256+EECDH:AES256+EDH;
        # ssl_session_cache shared:SSL:10m;
        # ssl_session_timeout 1d;
        # ssl_session_tickets off;

        location / {
            proxy_pass http://admin.web:3000;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }

    server {
        listen 80;
        listen [::]:80; # ensures that the server listens on both IPv4 and IPv6 on port 80.

        server_name ybzone.io.vn;  # Added the missing semicolon

        root /var/www/html;
        index index.html index.htm index.nginx-debian.html;

        location / {
            proxy_pass http://client.web:3000;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
        }
    }

    server {
        listen 80;
        listen [::]:80; # ensures that the server listens on both IPv4 and IPv6 on port 80.

        server_name admin.ybzone.io.vn;  # Added the missing semicolon

        root /var/www/html;
        index index.html index.htm index.nginx-debian.html;

        location / {    
            proxy_pass http://admin.web:3000;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
        }
    }
}
```