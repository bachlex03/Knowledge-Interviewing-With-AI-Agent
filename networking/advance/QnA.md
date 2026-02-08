en: Q1. Explain the difference between a routing protocol and a routed protocol.
vi: Q1. Giải thích sự khác nhau giữa routing protocol và routed protocol.
en: A1. A routing protocol exchanges route information (e.g., OSPF, BGP), while a routed protocol carries user data (e.g., IP).
vi: A1. Routing protocol trao đổi thông tin định tuyến (ví dụ: OSPF, BGP), còn routed protocol mang dữ liệu người dùng (ví dụ: IP).

en: Q2. What is BGP and when is it used?
vi: Q2. BGP là gì và khi nào dùng?
en: A2. BGP is the Internet’s inter-domain routing protocol used between autonomous systems to exchange routes and apply policy.
vi: A2. BGP là giao thức định tuyến liên miền của Internet, dùng giữa các hệ tự trị để trao đổi route và áp chính sách.

en: Q3. Describe NAT and one common drawback.
vi: Q3. Mô tả NAT và một nhược điểm phổ biến.
en: A3. NAT translates private addresses to public ones; a drawback is it breaks end-to-end connectivity and can complicate peer-to-peer protocols.
vi: A3. NAT chuyển đổi địa chỉ private sang public; nhược điểm là phá vỡ kết nối đầu-cuối và gây khó cho các giao thức P2P.

en: Q4. What is MTU and how does PMTUD work?
vi: Q4. MTU là gì và PMTUD hoạt động thế nào?
en: A4. MTU is the maximum frame size on a link; PMTUD discovers the smallest MTU along a path using ICMP feedback or probing.
vi: A4. MTU là kích thước khung tối đa trên một liên kết; PMTUD tìm MTU nhỏ nhất trên đường đi bằng phản hồi ICMP hoặc probing.

en: Q5. Explain the TCP three-way handshake and its purpose.
vi: Q5. Giải thích bắt tay 3 bước của TCP và mục đích.
en: A5. SYN, SYN-ACK, ACK establishes initial sequence numbers and confirms both sides are ready to communicate.
vi: A5. SYN, SYN-ACK, ACK thiết lập số thứ tự ban đầu và xác nhận hai phía sẵn sàng giao tiếp.

en: Q6. What is a VLAN and why is it used?
vi: Q6. VLAN là gì và tại sao dùng?
en: A6. A VLAN is a logical Layer-2 segmentation that isolates broadcast domains for security and traffic management.
vi: A6. VLAN là phân đoạn logic tầng 2 để cô lập miền broadcast phục vụ bảo mật và quản lý lưu lượng.

en: Q7. Compare TLS termination at a load balancer vs. end-to-end TLS.
vi: Q7. So sánh TLS termination tại load balancer và TLS end-to-end.
en: A7. Termination at the load balancer simplifies cert management and enables L7 routing but exposes plaintext inside; end-to-end keeps encryption to the backend with higher complexity.
vi: A7. Termination tại load balancer giúp quản lý chứng chỉ và định tuyến L7 dễ hơn nhưng lộ plaintext bên trong; end-to-end giữ mã hóa đến backend với độ phức tạp cao hơn.

en: Q8. What is the difference between congestion control and flow control in TCP?
vi: Q8. Sự khác nhau giữa congestion control và flow control trong TCP là gì?
en: A8. Flow control protects the receiver from being overwhelmed; congestion control protects the network from overload.
vi: A8. Flow control bảo vệ bên nhận khỏi quá tải; congestion control bảo vệ mạng khỏi tắc nghẽn.

en: Q9. Explain how DHCP works at a high level.
vi: Q9. Giải thích DHCP hoạt động ở mức khái quát.
en: A9. DHCP uses a DORA exchange (Discover, Offer, Request, Ack) to lease IP configuration to clients.
vi: A9. DHCP dùng chuỗi DORA (Discover, Offer, Request, Ack) để cấp lease cấu hình IP cho client.

en: Q10. When would you prefer UDP over TCP in a system design?
vi: Q10. Khi nào bạn ưu tiên UDP hơn TCP trong thiết kế hệ thống?
en: A10. Prefer UDP for latency-sensitive or loss-tolerant traffic like VoIP, live streaming, or real-time telemetry.
vi: A10. Ưu tiên UDP cho lưu lượng nhạy độ trễ hoặc chấp nhận mất gói như VoIP, live streaming hoặc telemetry thời gian thực.

en: Q11. Describe the purpose of ARP and a security risk associated with it.
vi: Q11. Mục đích của ARP là gì và một rủi ro bảo mật liên quan?
en: A11. ARP maps IP to MAC on local networks; ARP spoofing can redirect traffic to an attacker.
vi: A11. ARP ánh xạ IP sang MAC trong mạng cục bộ; ARP spoofing có thể chuyển hướng lưu lượng đến kẻ tấn công.

en: Q12. Explain what a default route is and why it matters.
vi: Q12. Giải thích default route là gì và vì sao quan trọng.
en: A12. A default route is the fallback path for destinations not in the routing table, enabling outbound connectivity.
vi: A12. Default route là đường đi dự phòng cho đích không có trong bảng định tuyến, giúp kết nối ra ngoài.
