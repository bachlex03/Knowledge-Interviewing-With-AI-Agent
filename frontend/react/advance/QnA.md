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
