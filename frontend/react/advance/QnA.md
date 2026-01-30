# React Advanced Q&A

## Q1: What is the Context API and when should you use it?
en: The Context API is a way for a React app to effectively produce global variables that can be passed around. This is the alternative to "prop drilling" or moving props from grandparent to child to parent, and so on. Use it when data needs to be accessible by many components at different nesting levels.
vi: Context API là một cách để ứng dụng React tạo ra các biến toàn cục một cách hiệu quả để có thể truyền đi khắp nơi. Đây là giải pháp thay thế cho việc "truyền props tầng tầng lớp lớp" (prop drilling) hoặc di chuyển props từ ông bà sang con cái sang cha mẹ, v.v. Hãy sử dụng nó khi dữ liệu cần được truy cập bởi nhiều thành phần ở các mức lồng nhau khác nhau.

---

## Q2: Explain the useMemo and useCallback hooks.
en: useMemo returns a memoized value, recomputing it only when dependencies change, useful for expensive calculations. useCallback returns a memoized version of a callback function that only changes if one of the dependencies has changed, useful when passing callbacks to optimized child components that rely on reference equality to prevent unnecessary renders.
vi: useMemo trả về một giá trị đã được ghi nhớ (memoized value), chỉ tính toán lại khi các phụ thuộc thay đổi, hữu ích cho các phép toán tốn kém. useCallback trả về một phiên bản ghi nhớ của một hàm gọi lại (callback function) và chỉ thay đổi nếu một trong các phụ thuộc đã thay đổi, hữu ích khi truyền các callback cho các thành phần con đã được tối ưu hóa dựa trên sự bằng nhau về tham chiếu để ngăn việc vẽ lại không cần thiết.

---

## Q3: What are Higher-Order Components (HOC)?
en: A higher-order component is a function that takes a component and returns a new component. It's a pattern derived from React's compositional nature for reusing component logic.
vi: Một thành phần bậc cao (Higher-Order Component) là một hàm nhận vào một thành phần và trả về một thành phần mới. Đó là một mẫu thiết kế bắt nguồn từ tính chất cấu thành (compositional nature) của React để tái sử dụng logic của thành phần.

---

## Q4: How do Error Boundaries work in React?
en: Error boundaries are React components that catch JavaScript errors anywhere in their child component tree, log those errors, and display a fallback UI instead of the component tree that crashed. They catch errors during rendering, in lifecycle methods, and in constructors of the whole tree below them.
vi: Biên lỗi (Error boundaries) là các thành phần React giúp bắt các lỗi JavaScript ở bất kỳ đâu trong cây thành phần con của chúng, ghi nhật ký các lỗi đó và hiển thị một giao diện dự phòng thay vì cây thành phần đã bị hỏng. Chúng bắt lỗi trong quá trình vẽ (rendering), trong các phương thức vòng đời và trong các hàm khởi tạo (constructors) của toàn bộ cây bên dưới chúng.

---

## Q5: What is React reconciliation?
en: Reconciliation is the process through which React updates the DOM. When a component's state or props change, React creates a new virtual DOM tree and compares it with the previous one (diffing). It then calculates the minimum number of changes needed to update the real DOM to match the new virtual DOM.
vi: Đối soát (Reconciliation) là quá trình mà qua đó React cập nhật DOM. Khi trạng thái hoặc props của một thành phần thay đổi, React tạo ra một cây DOM ảo mới và so sánh nó với cây trước đó (diffing). Sau đó, nó tính toán số lượng thay đổi tối thiểu cần thiết để cập nhật DOM thực sao cho khớp với DOM ảo mới.
---

## Q6: What are React Portals?
en: Portals provide a first-class way to render children into a DOM node that exists outside the DOM hierarchy of the parent component. This is commonly used for things like modals, tooltips, and floating menus.
vi: Portals cung cấp một cách tối ưu để render các phần tử con vào một nút DOM tồn tại bên ngoài hệ thống phân cấp DOM của thành phần cha. Điều này thường được sử dụng cho những thứ như modal, tooltip và menu thả nổi.

---

## Q7: What is the purpose of the React Profiler?
en: The Profiler measures how often a React application renders and what the "cost" of rendering is. Its purpose is to help identify parts of an application that are slow and may benefit from optimizations such as memoization.
vi: Profiler đo lường tần suất ứng dụng React render và "chi phí" của việc render là bao nhiêu. Mục đích của nó là giúp xác định các phần của ứng dụng bị chậm và có thể được hưởng lợi từ các tối ưu hóa như ghi nhớ (memoization).

---

## Q8: When and why would you create a Custom Hook?
en: You should create a custom hook when you want to extract component logic into reusable functions. Custom hooks allow you to share logic between multiple components without repeating code, especially when dealing with complex state management or side effects.
vi: Bạn nên tạo một hook tùy chỉnh khi muốn trích xuất logic của thành phần vào các hàm có thể tái sử dụng. Các hook tùy chỉnh cho phép bạn chia sẻ logic giữa nhiều thành phần mà không cần lặp lại mã, đặc biệt là khi xử lý quản lý trạng thái phức tạp hoặc các tác vụ lề (side effects).

---

## Q9: How do React.lazy and Suspense work together?
en: `React.lazy` lets you define a component that is loaded dynamically. This helps reduce the initial bundle size. `Suspense` is a component that wraps the lazy component and allows you to show a fallback UI (like a loading spinner) while waiting for the lazy component to load.
vi: `React.lazy` cho phép bạn định nghĩa một thành phần được tải động. Điều này giúp giảm kích thước gói ban đầu. `Suspense` là một thành phần bao bọc thành phần lazy và cho phép bạn hiển thị giao diện dự phòng (như biểu tượng đang tải) trong khi chờ thành phần lazy tải xong.

---

## Q10: What are Synthetic Events in React?
en: Synthetic Events are React's cross-browser wrapper around the browser’s native events. They have the same interface as native events, including `stopPropagation()` and `preventDefault()`, but they work identically across all browsers.
vi: Sự kiện tổng hợp (Synthetic Events) là lớp bao bọc của React xung quanh các sự kiện gốc của trình duyệt để hỗ trợ đa trình duyệt. Chúng có cùng giao diện với các sự kiện gốc, bao gồm `stopPropagation()` và `preventDefault()`, nhưng chúng hoạt động giống hệt nhau trên tất cả các trình duyệt.
