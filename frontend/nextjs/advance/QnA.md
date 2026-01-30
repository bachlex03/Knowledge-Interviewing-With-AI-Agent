# Next.js Advanced Q&A

## Q1: Explain Incremental Static Regeneration (ISR).
en: ISR allows you to create or update static pages after you’ve built your site. You can use it to regenerate specific pages in the background as traffic comes in, using a `revalidate` time or on-demand revalidation.
vi: ISR cho phép bạn tạo hoặc cập nhật các trang tĩnh sau khi bạn đã build trang web của mình. Bạn có thể sử dụng nó để tạo lại các trang cụ thể trong nền khi có truy cập, bằng cách sử dụng thời gian `revalidate` hoặc revalidation theo yêu cầu (on-demand).

---

## Q2: How does Data Fetching work in the App Router?
en: In the App Router, you can fetch data directly inside Server Components using `fetch` with extended options for caching and revalidating. This eliminates the need for `getStaticProps` or `getServerSideProps` used in the Pages Router.
vi: Trong App Router, bạn có thể lấy dữ liệu trực tiếp bên trong Server Components bằng cách sử dụng `fetch` với các tùy chọn mở rộng cho việc lưu bộ nhớ đệm (caching) và xác thực lại (revalidating). Điều này loại bỏ nhu cầu sử dụng `getStaticProps` hoặc `getServerSideProps` như trong Pages Router.

---

## Q3: What are Route Handlers?
en: Route Handlers allow you to create custom request handlers for a given route using the web Request and Response APIs. They are the App Router equivalent of API Routes and are defined in a `route.js` file.
vi: Route Handlers cho phép bạn tạo các trình xử lý yêu cầu (request handlers) tùy chỉnh cho một tuyến đường nhất định bằng cách sử dụng các API Web Request và Response. Chúng là phiên bản tương đương của API Routes trong App Router và được định nghĩa trong tệp `route.js`.

---

## Q4: Explain the 'Middleware' concept in Next.js.
en: Middleware allows you to run code before a request is completed. Based on the incoming request, you can modify the response by rewriting, redirecting, modifying the request or response headers, or responding directly.
vi: Middleware cho phép bạn chạy mã trước khi một yêu cầu được hoàn thành. Dựa trên yêu cầu đến, bạn có thể sửa đổi phản hồi bằng cách viết lại (rewriting), chuyển hướng (redirecting), sửa đổi các header của yêu cầu hoặc phản hồi, hoặc phản hồi trực tiếp.

---

## Q5: How do you optimize images in Next.js?
en: Next.js provides the `<Image>` component which automatically optimizes images for size, format, and resolution. It uses lazy loading by default and prevents Layout Shift by requiring width and height attributes or a blur placeholder.
vi: Next.js cung cấp thành phần `<Image>` tự động tối ưu hóa hình ảnh về kích thước, định dạng và độ phân giải. Nó sử dụng tải chậm (lazy loading) theo mặc định và ngăn chặn sự thay đổi bố cục (Layout Shift) bằng cách yêu cầu các thuộc tính chiều rộng và chiều cao hoặc một hình ảnh râm giữ chỗ (blur placeholder).
---

## Q6: How does 'Streaming' work in the App Router?
en: Streaming allows you to break down the page's HTML into smaller chunks and progressively send them to the client. This allows the user to see parts of the page before the entire content has finished loading, often implemented using `loading.js` or React `<Suspense>`.
vi: Streaming cho phép bạn chia nhỏ HTML của trang thành các phần nhỏ hơn và gửi chúng dần dần đến client. Điều này cho phép người dùng thấy được một phần của trang trước khi toàn bộ nội dung tải xong, thường được thực hiện bằng cách sử dụng `loading.js` hoặc React `<Suspense>`.

---

## Q7: What are 'Parallel Routes'?
en: Parallel Routes allow you to simultaneously or conditionally render one or more pages in the same layout. They are useful for dashboards or social feeds where sections can load independently.
vi: Tuyến đường song song (Parallel Routes) cho phép bạn render đồng thời hoặc có điều kiện một hoặc nhiều trang trong cùng một layout. Chúng hữu ích cho các bảng điều khiển (dashboards) hoặc các bảng tin xã hội nơi các phần có thể tải độc lập.

---

## Q8: What are 'Intercepting Routes'?
en: Intercepting Routes allow you to load a route from another part of your application within the current layout. This is useful when you want to show a route as a modal while keeping the context of the current page (e.g., viewing a photo in a feed).
vi: Tuyến đường chặn (Intercepting Routes) cho phép bạn tải một route từ một phần khác của ứng dụng ngay trong layout hiện tại. Điều này hữu ích khi bạn muốn hiển thị một route dưới dạng modal trong khi vẫn giữ ngữ cảnh của trang hiện tại (ví dụ: xem một bức ảnh trong một bảng tin).

---

## Q9: How does 'Automatic Code Splitting' work in Next.js?
en: Next.js automatically splits your code into smaller bundles. Only the JavaScript needed for the specific page you are visiting is loaded. This reduces the initial load time of the application.
vi: Next.js tự động chia nhỏ mã của bạn thành các gói (bundles) nhỏ hơn. Chỉ có mã JavaScript cần thiết cho trang cụ thể mà bạn đang truy cập mới được tải. Điều này giúp giảm thời gian tải ban đầu của ứng dụng.

---

## Q10: Explain 'Draft Mode' in Next.js.
en: Draft Mode allows you to preview headless CMS content in real-time. By enabling Draft Mode, Next.js will bypass the cache and fetch data at request time, allowing you to see the latest changes before they are published.
vi: Chế độ bản nháp (Draft Mode) cho phép bạn xem trước nội dung từ headless CMS trong thời gian thực. Bằng cách kích hoạt Draft Mode, Next.js sẽ bỏ qua bộ nhớ đệm và lấy dữ liệu tại thời điểm yêu cầu, cho phép bạn thấy các thay đổi mới nhất trước khi chúng được xuất bản.
