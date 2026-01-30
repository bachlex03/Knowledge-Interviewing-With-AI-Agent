# Next.js Foundation Q&A

## Q1: What is Next.js and how does it differ from React? 
en: Next.js is a React framework that provides building blocks to create web applications. While React is a library for building user interfaces, Next.js handles the tooling and configuration needed for React, and provides additional features like server-side rendering, static site generation, and routing out of the box.
vi: Next.js là một framework React cung cấp các khối xây dựng (building blocks) để tạo các ứng dụng web. Trong khi React là một thư viện để xây dựng giao diện người dùng, Next.js xử lý việc thiết lập công cụ và cấu hình cần thiết cho React, đồng thời cung cấp các tính năng bổ sung như render phía máy chủ (SSR), tạo trang tĩnh (SSG) và định tuyến (routing) có sẵn.

---

## Q2: What is the difference between Server Components and Client Components? - **HIGHT**
en: Server Components (default in App Router) are rendered on the server and never sent to the client, reducing the JavaScript bundle size. Client Components are rendered on the client and are used for interactivity (e.g., using useState, useEffect, or event listeners).
vi: Server Components (mặc định trong App Router) được render trên máy chủ và không bao giờ được gửi đến client, giúp giảm kích thước gói JavaScript. Client Components được render trên máy khách và được sử dụng cho tính tương tác (ví dụ: sử dụng useState, useEffect hoặc các trình xử lý sự kiện).

### Note: Context on "Server" vs "Client"

en:
*   **Server**: This refers to the backend infrastructure where your application code executes before reaching the user. **Server Components** render here, allowing secure data fetching and rendering HTML without sending logic to the browser.
*   **Client**: This refers to the user's web browser running on their device. **Client Components** are sent to the browser to handle interactivity (state, effects, events).

vi:
*   **Server (Máy chủ)**: Đây là hạ tầng backend nơi mã ứng dụng thực thi trước khi đến tay người dùng. **Server Components** được render tại đây, cho phép lấy dữ liệu an toàn và render HTML mà không gửi logic xuống trình duyệt.
*   **Client (Máy khách)**: Đây là trình duyệt web của người dùng chạy trên thiết bị của họ. **Client Components** được gửi đến trình duyệt để xử lý tính tương tác (trạng thái, hiệu ứng, sự kiện).

---

## Q3: Explain Static Site Generation (SSG).
en: Static Site Generation is a pre-rendering method where the HTML is generated at build time. The same HTML is then reused on each request. It is very fast and great for SEO.
vi: Static Site Generation (Tạo trang tĩnh) là một phương pháp pre-rendering trong đó HTML được tạo ra tại thời điểm build. Cùng một bản HTML đó sau đó được sử dụng lại cho mỗi yêu cầu. Nó rất nhanh và tuyệt vời cho SEO.

---

## Q4: How does file-based routing work in Next.js?
en: Next.js uses the file system to define routes. In the App Router, files named `page.js` (or `.tsx`) inside folders in the `app/` directory automatically become routes matching the folder structure.
vi: Next.js sử dụng hệ thống tệp để xác định các tuyến đường (routes). Trong App Router, các tệp có tên `page.js` (hoặc `.tsx`) bên trong các thư mục trong thư mục `app/` sẽ tự động trở thành các routes khớp với cấu trúc thư mục đó.

---

## Q5: What is the purpose of the 'Link' component?
en: The `<Link>` component is used for client-side navigation between routes. It pre-fetches the linked page's data in the background, making transitions feel instantaneous without a full page reload.
vi: Thành phần `<Link>` được sử dụng để điều hướng phía client giữa các routes. Nó thực hiện tải trước (pre-fetches) dữ liệu của trang được liên kết trong nền, giúp việc chuyển trang có cảm giác tức thì mà không cần tải lại toàn bộ trang.
---

## Q6: What is the 'public' folder used for in Next.js?
en: The `public` folder is used to store static assets like images, fonts, and robots.txt. Files inside this folder can be referenced starting from the base URL (e.g., `/logo.png`).
vi: Thư mục `public` được sử dụng để lưu trữ các tài sản tĩnh như hình ảnh, font chữ và tệp robots.txt. Các tệp bên trong thư mục này có thể được tham chiếu bắt đầu từ URL cơ sở (ví dụ: `/logo.png`).

---

## Q7: How do Dynamic Routes work?
en: Dynamic Routes allow you to create pages with dynamic segments by wrapping a folder name in square brackets, like `[id]`. This segment will be passed as a prop to the page component.
vi: Tuyến đường động (Dynamic Routes) cho phép bạn tạo các trang có các phân đoạn động bằng cách bao bọc tên thư mục trong dấu ngoặc vuông, chẳng hạn như `[id]`. Phân đoạn này sẽ được truyền dưới dạng một prop cho thành phần trang.

---

## Q8: What is the metadata API in Next.js?
en: Next.js provides a Metadata API that allows you to define your application metadata (e.g., title, description) for improved SEO and web shareability. It can be defined statically or dynamically in `layout.js` or `page.js`.
vi: Next.js cung cấp một API Metadata cho phép bạn định nghĩa siêu dữ liệu cho ứng dụng của mình (ví dụ: tiêu đề, mô tả) để cải thiện SEO và khả năng chia sẻ trên web. Nó có thể được định nghĩa tĩnh hoặc động trong tệp `layout.js` hoặc `page.js`.

---

## Q9: Explain the concept of 'Layouts' in Next.js.
en: A layout is UI that is shared between multiple pages. Layouts preserve state, remain interactive, and do not re-render upon navigation. They are defined using a `layout.js` file.
vi: Layout là giao diện người dùng (UI) được chia sẻ giữa nhiều trang. Layout giữ nguyên trạng thái, duy trì tính tương tác và không vẽ lại (re-render) khi điều hướng. Chúng được định nghĩa bằng tệp `layout.js`.

---

## Q10: What is 'Fast Refresh'?
en: Fast Refresh is a Next.js feature that gives you instantaneous feedback on edits made to your React components. It only re-renders the component that was modified without losing the application state.
vi: Fast Refresh là một tính năng của Next.js giúp bạn nhận được phản hồi tức thì về các chỉnh sửa được thực hiện trên các thành phần React. Nó chỉ vẽ lại thành phần đã được sửa đổi mà không làm mất trạng thái của ứng dụng.
