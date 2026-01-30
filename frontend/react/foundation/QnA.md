# React Foundation Q&A

## Q1: What is JSX?
en: JSX stands for JavaScript XML. It is a syntax extension for JavaScript that allows you to write HTML-like code within your JavaScript files. It makes it easier to describe what the UI should look like.
vi: JSX (JavaScript XML) là một phần mở rộng cú pháp cho JavaScript, cho phép bạn viết mã giống như HTML ngay trong các tệp JavaScript. Nó giúp việc mô tả giao diện người dùng (UI) trở nên dễ dàng hơn.

---

## Q2: What is the difference between Functional and Class components?
en: Functional components are just plain JavaScript functions that accept props as an argument and return a React element. Class components are ES6 classes that extend from React.Component and must have a render() method. Since React 16.8 (Hooks), functional components can also manage state and side effects.
vi: Thành phần hàm (Functional components) chỉ là các hàm JavaScript thông thường nhận props làm đối số và trả về một phần tử React. Thành phần lớp (Class components) là các lớp ES6 kế thừa từ React.Component và phải có phương thức render(). Kể từ phiên bản React 16.8 (Hooks), các thành phần hàm cũng có thể quản lý trạng thái (state) và các tác vụ lề (side effects).

---

## Q3: What are Props and State?
en: Props (short for "properties") are read-only inputs passed from a parent component to a child component. State is a local data storage that is managed within the component itself and can change over time, triggering a re-render.
vi: Props (viết tắt của "properties") là các đầu vào chỉ đọc được truyền từ thành phần cha sang thành phần con. Trạng thái (State) là kho lưu trữ dữ liệu cục bộ được quản lý bên trong chính thành phần đó và có thể thay đổi theo thời gian, kích hoạt việc vẽ lại (re-render).

---

## Q4: How do you handle events in React?
en: Handling events in React is similar to handling events on DOM elements, but with some syntax differences: Events are named using camelCase (e.g., onClick instead of onclick) and you pass a function as the event handler rather than a string.
vi: Việc xử lý sự kiện trong React tương tự như xử lý sự kiện trên các phần tử DOM, nhưng có một số khác biệt về cú pháp: Các sự kiện được đặt tên theo kiểu camelCase (ví dụ: onClick thay vì onclick) và bạn truyền một hàm làm trình xử lý sự kiện thay vì một chuỗi.

---

## Q5: Why is the 'key' prop important in lists?
en: Keys help React identify which items have changed, been added, or been removed. They should be given to the elements inside the array to give the elements a stable identity, which helps in efficient DOM updates during reconciliation.
vi: Các khóa (Keys) giúp React xác định mục nào đã thay đổi, được thêm vào hoặc bị xóa bỏ. Chúng nên được gán cho các phần tử bên trong mảng để tạo cho chúng một danh tính ổn định, giúp cập nhật DOM hiệu quả trong quá trình đối soát (reconciliation).
---

## Q6: What is the Virtual DOM and how does it work?
en: The Virtual DOM is a lightweight, in-memory representation of the real DOM. When the state of an object changes, React updates the virtual DOM first. Then, it compares the current virtual DOM with a previous version (diffing) and only updates the necessary parts in the real DOM.
vi: Virtual DOM là một đại diện nhẹ, nằm trong bộ nhớ của Real DOM. Khi trạng thái của một đối tượng thay đổi, React sẽ cập nhật Virtual DOM trước. Sau đó, nó so sánh Virtual DOM hiện tại với phiên bản trước đó (diffing) và chỉ cập nhật những phần cần thiết trong Real DOM.
