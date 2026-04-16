**Question 1:** Hãy so sánh JWT và Cookie và Session

**Answer:**

## Tổng quan kiến trúc

|               | JWT                                         | Cookie                  | Session                      |
| ------------- | ------------------------------------------- | ----------------------- | ---------------------------- |
| **Storage**   | Client-side (localStorage / cookie)         | Client-side             | Server-side                  |
| **State**     | Stateless                                   | Stateless/Stateful      | Stateful                     |
| **Transport** | `Authorization: Bearer <token>` hoặc Cookie | `Cookie` header tự động | `Cookie` header (session ID) |

---

## 1. Session

**Server-side state management.** Server lưu trữ session data trong memory store (Redis, Memcached) hoặc database.

```
Client                          Server
  |                               |
  |-- POST /login --------------> |
  |                           [Tạo session]
  |                           session_store["abc123"] = { userId: 1 }
  |<-- Set-Cookie: sid=abc123 --- |
  |                               |
  |-- GET /profile (sid=abc123)-> |
  |                           [Lookup session_store["abc123"]]
  |<-- 200 OK ------------------- |
```

**Đặc điểm kỹ thuật:**

- Session ID là một **opaque token** (random string), không chứa data
- Mỗi request cần **I/O round-trip** đến session store → bottleneck ở scale lớn
- Invalidation tức thì: xóa key khỏi store là xong
- Vulnerable: **Session Fixation**, **Session Hijacking**

---

## 2. Cookie

Cookie là **transport/storage mechanism**, không phải auth protocol. Thường dùng để mang session ID hoặc JWT.

```http
Set-Cookie: token=xyz; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600
```

**Các flag quan trọng:**

| Flag                  | Mục đích                               |
| --------------------- | -------------------------------------- |
| `HttpOnly`            | Ngăn JavaScript đọc cookie → chống XSS |
| `Secure`              | Chỉ gửi qua HTTPS                      |
| `SameSite=Strict/Lax` | Chống CSRF                             |
| `Max-Age` / `Expires` | TTL của cookie                         |

Cookie **tự động được đính kèm** vào mọi request cùng domain → dễ dùng nhưng cần chú ý CSRF.

---

## 3. JWT (JSON Web Token)

**Self-contained, stateless token.** Cấu trúc gồm 3 phần base64url-encoded, phân cách bởi dấu `.`:

```
Header.Payload.Signature
eyJhbGciOiJIUzI1NiJ9.eyJ1c2VySWQiOjF9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

**Header:** chỉ định thuật toán ký

```json
{ "alg": "RS256", "typ": "JWT" }
```

**Payload (Claims):**

```json
{
  "sub": "user_id_123",
  "iat": 1713200000,
  "exp": 1713203600,
  "roles": ["admin"]
}
```

**Signature** (với RS256):

```
RSASHA256(base64(header) + "." + base64(payload), privateKey)
```

Server **verify chữ ký** mà không cần lookup database → truly stateless.

**Lưu ý quan trọng:**

- Payload chỉ được **encode, không encrypt** → đừng lưu sensitive data (dùng JWE nếu cần)
- **Không thể revoke** trước khi hết hạn (trừ khi dùng blacklist → mất stateless)
- Nên dùng **RS256** (asymmetric) thay vì HS256 ở microservices, để service con verify mà không cần biết secret

---

## So sánh kỹ thuật chuyên sâu

| Tiêu chí                | JWT                                         | Session                                   |
| ----------------------- | ------------------------------------------- | ----------------------------------------- |
| **Scalability**         | Horizontal scale dễ (no shared state)       | Cần sticky session hoặc centralized store |
| **Revocation**          | Khó (cần blacklist)                         | Dễ (xóa session)                          |
| **Payload size**        | Lớn hơn (~200–500 bytes)                    | Nhỏ (chỉ ID ~32 bytes)                    |
| **DB lookup/request**   | 0                                           | 1 (session store)                         |
| **Security risk chính** | Token theft (XSS nếu lưu localStorage)      | Session hijacking, CSRF                   |
| **Microservices**       | Phù hợp (stateless, verify bằng public key) | Không phù hợp (cần share session store)   |

---

## Khi nào dùng gì?

- **Session** → Monolith, cần revoke ngay lập tức (banking, admin)
- **JWT** → Microservices, stateless API, mobile client, cross-domain auth
- **Cookie** → Luôn dùng làm transport cho cả 2 trường hợp trên (an toàn hơn localStorage vì có `HttpOnly`)

> **Best practice:** Dùng JWT lưu trong `HttpOnly Cookie` + `SameSite=Strict` + short expiry + refresh token rotation — tận dụng stateless của JWT đồng thời giảm thiểu XSS/CSRF risk.

===============

**Question 2:** "tận dụng stateless của JWT" là sao?

**Answer:**

### Vấn đề của Session (Stateful)

Mỗi request đến, server **bắt buộc phải hỏi** session store:

```
Request → Server A → Redis ("session abc123 có hợp lệ không?") → trả kết quả
```

Nếu có 3 server, cả 3 đều phải **kết nối chung 1 Redis**. Scale lên 100 server → 100 connection tranh nhau hỏi Redis → bottleneck.

---

### JWT giải quyết bằng cách nào?

JWT **nhúng toàn bộ thông tin cần thiết vào trong token**, và được **ký số (signed)**. Server chỉ cần:

```
1. Giải mã token
2. Verify chữ ký  ← chỉ là phép toán crypto, không cần I/O
3. Đọc payload
```

Không cần gọi database, không cần Redis, không cần hỏi ai cả.

```
Request + JWT → Server A → verify signature (CPU only) → done ✅
Request + JWT → Server B → verify signature (CPU only) → done ✅
Request + JWT → Server C → verify signature (CPU only) → done ✅
```

Ba server **hoàn toàn độc lập**, không share state → đây là **stateless**.

---

### Analogy kỹ thuật

|            | Session                                                 | JWT                                                   |
| ---------- | ------------------------------------------------------- | ----------------------------------------------------- |
| Giống như  | Thẻ giữ xe — bảo vệ phải tra sổ mới biết xe nào của bạn | CCCD — thông tin in sẵn, ai cũng verify được bằng mắt |
| Server cần | Lookup external store                                   | Chỉ cần public key (đã có sẵn)                        |

---

### Cụ thể trong Microservices

Với RS256, `auth-service` ký bằng **private key**, các service khác verify bằng **public key**:

```
auth-service (có private key)
    └── ký JWT, trả về client

order-service (chỉ có public key)
    └── nhận JWT → verify chữ ký → đọc userId, roles
    └── KHÔNG cần gọi auth-service ← đây là "tận dụng stateless"

inventory-service (chỉ có public key)
    └── làm tương tự, hoàn toàn độc lập
```

Nếu dùng Session, `order-service` và `inventory-service` đều phải gọi về auth-service hoặc share Redis → **coupling chặt**, **single point of failure**.

---

### Tóm lại

> **"Tận dụng stateless"** = server không cần lưu hay tra cứu bất cứ thứ gì để xác thực — mọi thông tin đã được **self-contained trong token** và đảm bảo toàn vẹn bằng **chữ ký số**. Kết quả là horizontal scaling gần như miễn phí về mặt auth.
