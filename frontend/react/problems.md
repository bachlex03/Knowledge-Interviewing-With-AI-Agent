# React Mastery Roadmap: Problems & Concepts

This document summarizes the core problems and advanced concepts that a React developer needs to master to reach a Senior/Architect level.

---

## 1. Core Runtime & Rendering (Nền tảng Thực thi & Render)
en: Understanding the engine behind the UI to prevent bugs and performance issues.
vi: Hiểu về động cơ đằng sau giao diện để ngăn chặn lỗi và các vấn đề hiệu suất.

- **When a component re-renders?**: Understanding triggers (State, Props, Parent, Context).
- **React Component Lifecycle**: Mastery of Mount, Update, and Unmount phases (Hooks equivalent).
- **Virtual DOM & Reconciliation**: How the Diffing algorithm works to minimize DOM updates.
- **Synthetic Events**: How React normalizes events across different browsers.

---

## 2. Data Flow & Communication (Luồng dữ liệu & Giao tiếp)
en: Strategies for moving and managing data across the component tree.
vi: Các chiến lược để di chuyển và quản lý dữ liệu xuyên suốt cây thành phần.

- **One-Way Data Flow (SSOT)**: Managing a Single Source of Truth to avoid synchronization bugs.
- **Props Drilling**: Identifying when it becomes a problem and how to avoid it.
- **2-Way Binding (Controlled vs Uncontrolled)**: When to let React control inputs vs. the DOM.
- **Passing Callback to Props**: Handling referential equality with stable function references.

---

## 3. Optimization & Resource Management (Tối ưu hóa & Quản lý Tài nguyên)
en: Keeping the application fast and efficient even with large data sets.
vi: Giữ cho ứng dụng nhanh và hiệu quả ngay cả với các tập dữ liệu lớn.

- **Cleanup in Functional Components**: Preventing memory leaks (timers, subscriptions, listeners).
- **Memoization Strategies**: Proper use of `React.memo`, `useMemo`, and `useCallback`.
- **Virtualization & Windowing**: Rendering only visible items in massive lists (e.g., millions of rows).
- **Asset Optimization**: Effective use of `React.lazy` and `Suspense` for code splitting.

---

## 4. Modern & Concurrent React (React Hiện đại & Đồng thời)
en: Leveraging the power of React 18+ for high-priority user interactions.
vi: Tận dụng sức mạnh của React 18+ cho các tương tác người dùng có độ ưu tiên cao.

- **Concurrent Rendering**: Non-blocking UI updates for better responsiveness.
- **React Tearing**: Preventing UI inconsistency when external stores update during concurrent renders.
- **Hydration Mismatch**: Fixing discrepancies between Server-Side Rendering (SSR) and Client-Side output.
- **Transitions**: Using `useTransition` and `useDeferredValue` to deprioritize heavy UI updates.

---

## 5. Architectural Patterns (Mẫu thiết kế Kiến trúc)
en: Building reusable, scalable, and maintainable component systems.
vi: Xây dựng các hệ thống thành phần có thể tái sử dụng, mở rộng và dễ bảo trì.

- **Higher-Order Components (HOC)**: Reusing logic by wrapping components.
- **Compound Components**: Creating flexible UI systems (e.g., Menu, Tabs) with implicit state.
- **Render Props & Slot Pattern**: Sharing UI structure and logic dynamically.
- **Custom Hooks**: Extracting complex side effects and state into reusable functions.
- **Server Components (RSC)**: Designing for the shift from Client to Server-centric rendering.

---

## 6. Advanced State & Safety (Quản lý Trạng thái & An toàn Nâng cao)
en: Handling complex business logic and securing the user interface.
vi: Xử lý logic nghiệp vụ phức tạp và bảo mật giao diện người dùng.

- **State Machines**: Managing complex interaction flows to avoid "Boolean Hell" (XState).
- **Micro-State Management**: Choosing between Atoms (Jotai) vs. Stores (Zustand/Redux).
- **Security (XSS Prevention)**: Safeguarding the app against script injections and untrusted HTML.
- **Side Effect Testing**: Mocking network requests, browser APIs, and time correctly in tests.

---

# Deep Dive: Problems & Solutions (Phân tích Chuyên sâu)

## 1. Compound Components Pattern
en: **Problem**: Creating flexible UI components (like Tabs or Accodions) often leads to "Prop Drilling" or rigid structures where the parent component has to manage all children's internal states through complex props.
vi: **Vấn đề**: Việc tạo các UI component linh hoạt (như Tabs hoặc Accordions) thường dẫn đến "Prop Drilling" hoặc cấu trúc cứng nhắc, nơi component cha phải quản lý tất cả trạng thái nội bộ của con thông qua các props phức tạp.

**Solution (Sử dụng Context API nội bộ):**

```javascript
import React, { useState, createContext, useContext } from 'react';

const TabsContext = createContext();

// 1. Parent Component
export function Tabs({ children, defaultValue }) {
  const [activeTab, setActiveTab] = useState(defaultValue);
  return (
    <TabsContext.Provider value={{ activeTab, setActiveTab }}>
      <div className="tabs-container">{children}</div>
    </TabsContext.Provider>
  );
}

// 2. Sub-components (Sub-items)
Tabs.List = ({ children }) => <div className="tab-list">{children}</div>;

Tabs.Trigger = ({ value, children }) => {
  const { activeTab, setActiveTab } = useContext(TabsContext);
  return (
    <button 
      className={activeTab === value ? 'active' : ''} 
      onClick={() => setActiveTab(value)}
    >
      {children}
    </button>
  );
};

Tabs.Content = ({ value, children }) => {
  const { activeTab } = useContext(TabsContext);
  return activeTab === value ? <div>{children}</div> : null;
};
```

---

## 2. Render Props & Slot Pattern
en: **Problem**: How to share **behavioral logic** or **UI structure** between components while remaining flexible. You want to avoid hardcoding UI inside logic-heavy components.
vi: **Vấn đề**: Làm thế nào để chia sẻ **logic hành vi** hoặc **cấu trúc UI** giữa các component trong khi vẫn giữ được sự linh hoạt. Bạn muốn tránh việc viết cứng (hardcode) giao diện bên trong các component chứa nhiều logic.

### A. Render Props (Sharing Behavior)
en: A component with a "render prop" takes a function that returns a React element and calls it instead of implementing its own render logic. It's essentially saying: *"I will handle the logic (state, events), you tell me how to draw it."*
vi: Một component với "render prop" nhận vào một hàm trả về một React element và gọi hàm đó thay vì tự triển khai logic hiển thị. Về cơ bản nó có nghĩa là: *"Tôi sẽ lo phần logic (state, sự kiện), bạn hãy cho tôi biết phải vẽ nó như thế nào."*

```javascript
// 1. Logic Component / Component chứa Logic
const MouseTracker = ({ render }) => {
  const [position, setPosition] = useState({ x: 0, y: 0 });
  const handleMouseMove = (e) => setPosition({ x: e.clientX, y: e.clientY });

  return (
    <div style={{ height: '200px', border: '1px solid' }} onMouseMove={handleMouseMove}>
      {/* en: Pass internal state to the render function / vi: Truyền state nội bộ vào hàm render */}
      {render(position)}
    </div>
  );
};

// 2. Usage / Cách dùng:
// en: User A wants text, User B wants a red dot. Both use the same logic!
// vi: Người dùng A muốn hiện chữ, người dùng B muốn hiện một chấm đỏ. Cả hai dùng chung logic!
const App = () => (
  <>
    <MouseTracker render={({ x, y }) => <p>Mouse: {x}, {y}</p>} />
    <MouseTracker render={({ x, y }) => (
      <div style={{ position: 'absolute', left: x, top: y, background: 'red', width: 10, height: 10 }} />
    )} />
  </>
);
```

### B. Slot Pattern (Sharing Structure)
en: Used mostly for **Layouts**. Instead of just one `children` prop, you define multiple "slots" (props) to place components in specific locations. This prevents the Parent from having to know about the internal structure of the children.
vi: Thường được dùng cho các **Layouts**. Thay vì chỉ có một prop `children` duy nhất, bạn định nghĩa nhiều "slots" (props) để đặt các component vào những vị trí cụ thể. Điều này giúp component Cha không cần biết về cấu trúc bên trong của các con.

```javascript
// 1. Layout Component / Component Layout
const PageLayout = ({ header, sidebar, content, footer }) => (
  <div className="layout">
    <header>{header}</header>
    <div className="main-area">
      <aside>{sidebar}</aside>
      <main>{content}</main>
    </div>
    <footer>{footer}</footer>
  </div>
);

// 2. Usage / Cách dùng:
// en: Clean and readable. No need to wrap everything in multiple divs.
// vi: Sạch sẽ và dễ đọc. Không cần bao bọc mọi thứ trong nhiều thẻ div lồng nhau.
<PageLayout 
  header={<Navbar />}
  sidebar={<Menu />}
  content={<Dashboard />}
  footer={<Copyright />}
/>
```

**Comparison / So sánh:**
en: 
- **Render Props**: Best for sharing **logic** (like tracking, data fetching) that is coupled with a DOM element.
- **Slot Pattern**: Best for **UI Layouts** that need clear organization better than `children`.
- **Note**: For pure logic (no DOM), **Custom Hooks** are usually preferred today over Render Props.

vi:
- **Render Props**: Tốt nhất để chia sẻ **logic** (như tracking, fetch dữ liệu) vốn đi kèm với một phần tử DOM.
- **Slot Pattern**: Tốt nhất cho các **UI Layouts** cần sự tổ chức rõ ràng hơn là dùng `children`.
- **Lưu ý**: Với logic thuần túy (không có DOM), **Custom Hooks** thường được ưu tiên hơn Render Props ngày nay.


---

## 3. Server Components (RSC) vs Client Components
en: **Problem**: Client-side hydration is expensive. We want to render as much as possible on the server to reduce JS bundle size.
vi: **Vấn đề**: Quá trình Hydration ở phía Client rất tốn kém. Chúng ta muốn render nhiều nhất có thể trên server để giảm kích thước gói JS.

**Solution (Next.js context):**

```javascript
// Server Component (Default in Next.js App Router)
// This code runs ONLY on the server.
async function ProductList() {
  const products = await db.query('SELECT * FROM products'); // Direct DB access!

  return (
    <ul>
      {products.map(p => (
        <li key={p.id}>
          {p.name} - <AddToCartButton id={p.id} /> 
        </li>
      ))}
    </ul>
  );
}

// Client Component (Explicitly marked)
'use client'; 
import { useState } from 'react';

function AddToCartButton({ id }) {
  const [added, setAdded] = useState(false); // Interactivity requires Client Component
  return <button onClick={() => setAdded(true)}>{added ? 'Added' : 'Add'}</button>;
}
```

---

## 4. Hydration Mismatch
en: **Problem**: Error "Text content did not match" occurs when the server-rendered HTML is different from the first client-side render (e.g., using `new Date()` or `Math.random()`).
vi: **Vấn đề**: Lỗi "Text content did not match" xảy ra khi HTML render từ server khác với lần render đầu tiên ở phía client (ví dụ: sử dụng `new Date()` hoặc `Math.random()`).

**Solution (Ensure synchronization):**

```javascript
function SafeDate() {
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  if (!isClient) return <span>Loading date...</span>;

  return <span>{new Date().toLocaleTimeString()}</span>;
}
```

---

## 5. State Machines (FSM)
en: **Problem**: Handling complex scenarios like "Loading -> Success -> Error" often results in "Boolean Hell" where states become inconsistent (e.g., `isLoading` and `isError` both being true).
vi: **Vấn đề**: Xử lý các kịch bản phức tạp giúp tránh tình trạng "Boolean Hell" - nơi các trạng thái trở nên bất nhất (ví dụ: cả `isLoading` và `isError` đều bằng true).

**Solution (Manual FSM):**

```javascript
const STAGES = { IDLE: 'idle', LOADING: 'loading', SUCCESS: 'success', ERROR: 'error' };

function Fetcher() {
  const [status, setStatus] = useState(STAGES.IDLE);
  const [data, setData] = useState(null);

  const fetchData = async () => {
    setStatus(STAGES.LOADING);
    try {
      const res = await api.call();
      setData(res);
      setStatus(STAGES.SUCCESS);
    } catch {
      setStatus(STAGES.ERROR);
    }
  };

  return (
    <div>
      {status === STAGES.LOADING && <Spinner />}
      {status === STAGES.SUCCESS && <DataView data={data} />}
      {status === STAGES.ERROR && <ErrorView />}
      <button onClick={fetchData}>Fetch</button>
    </div>
  );
}
```

---

## 6. Micro-State Management (Atomic State)
en: **Problem**: Re-rendering the entire app when a small piece of global state changes (e.g., in a large Redux store).
vi: **Vấn đề**: Toàn bộ ứng dụng bị render lại khi một phần nhỏ của trạng thái toàn cục thay đổi (ví dụ: trong một store Redux lớn).

**Solution (Using Zustand or Jotai atoms):**

```javascript
import { create } from 'zustand';

// Create a small, focused store
const useUserStore = create((set) => ({
  username: 'anonymous',
  updateUsername: (name) => set({ username: name }),
}));

function Profile() {
  const username = useUserStore((state) => state.username);
  return <h1>Hello, {username}</h1>;
}
```

---

## 7. Security (XSS / Data Injection)
en: **Problem**: Improperly rendering user-provided content using `dangerouslySetInnerHTML` can allow attackers to run malicious scripts.
vi: **Vấn đề**: Việc render nội dung do người dùng cung cấp không đúng cách bằng `dangerouslySetInnerHTML` có thể cho phép kẻ tấn công thực thi các script độc hại.

**Solution (Sanitization):**

```javascript
import DOMPurify from 'dompurify';

function UserComment({ rawHtmlFromUser }) {
  const cleanHtml = DOMPurify.sanitize(rawHtmlFromUser);
  return <div dangerouslySetInnerHTML={{ __html: cleanHtml }} />;
}
```

---

## 8. Virtualization & Windowing
en: **Problem**: Rendering 10,000 DOM nodes at once causes massive lag and memory usage.
vi: **Vấn đề**: Render 10.000 node DOM cùng lúc gây ra hiện tượng lag cực nặng và tốn bộ nhớ.

**Solution (Manual Concept):**

```javascript
function VirtualList({ items, itemHeight, viewportHeight }) {
  const [scrollTop, setScrollTop] = useState(0);

  const startIndex = Math.floor(scrollTop / itemHeight);
  const endIndex = startIndex + Math.ceil(viewportHeight / itemHeight);
  const visibleItems = items.slice(startIndex, endIndex);

  return (
    <div 
      onScroll={(e) => setScrollTop(e.target.scrollTop)} 
      style={{ height: viewportHeight, overflow: 'auto', position: 'relative' }}
    >
      <div style={{ height: items.length * itemHeight }}>
        <div style={{ position: 'absolute', top: startIndex * itemHeight }}>
          {visibleItems.map(item => <Row key={item.id} data={item} />)}
        </div>
      </div>
    </div>
  );
}
```

---

## 9. Asset Optimization (Image & Font)
en: **Problem**: Large images and many fonts slow down LCP (Largest Contentful Paint) and CLS (Cumulative Layout Shift).
vi: **Vấn đề**: Hình ảnh lớn và quá nhiều font chữ làm chậm LCP và gây ra CLS (Thay đổi bố cục lũy kế).

**Solution (Lazy loading & Priority):**

```javascript
function HeroSection() {
  return (
    <div>
      <img src="/hero.jpg" fetchpriority="high" alt="Banner" width="1200" height="600" />
      <img src="/feature.png" loading="lazy" alt="Feature" width="200" height="200" />
    </div>
  );
}
```

---

## 10. Concurrent Transitions (Non-blocking UI)
en: **Problem**: Updating state that causes a heavy re-render (like filtering a massive list) freezes the UI, making it unresponsive to user input (typing/clicking) until the render is finished.
vi: **Vấn đề**: Cập nhật trạng thái gây ra render nặng (như lọc một danh sách khổng lồ) làm đóng băng giao diện, khiến ứng dụng không phản hồi các thao tác của người dùng (gõ/click) cho đến khi quá trình render kết thúc.

**Solution (useTransition):**

```javascript
import { useState, useTransition } from 'react';

function SearchList({ items }) {
  const [isPending, startTransition] = useTransition();
  const [filterTerm, setFilterTerm] = useState('');
  const [filteredItems, setFilteredItems] = useState(items);

  const handleChange = (e) => {
    // en: Keep the input update high-priority (urgent)
    // vi: Giữ cho việc cập nhật ô input có độ ưu tiên cao (khẩn cấp)
    setFilterTerm(e.target.value);

    // en: Mark the heavy filtering as a transition (low-priority)
    // vi: Đánh dấu việc lọc dữ liệu nặng là một transition (độ ưu tiên thấp)
    startTransition(() => {
      const filtered = items.filter(item => item.includes(e.target.value));
      setFilteredItems(filtered);
    });
  };

  return (
    <div>
      <input type="text" onChange={handleChange} value={filterTerm} />
      {isPending ? <p>Updating list...</p> : <List data={filteredItems} />}
    </div>
  );
}
```

---

## 11. React Tearing (External Stores)
en: **Problem**: In Concurrent Mode, if an external store (like Redux or a global variable) updates *during* rendering, different parts of the UI might render different versions of the data, leading to a "torn" or inconsistent UI.
vi: **Vấn đề**: Trong Concurrent Mode, nếu một store bên ngoài (như Redux hoặc biến toàn cục) cập nhật *trong quá trình* render, các phần khác nhau của giao diện có thể hiển thị các phiên bản dữ liệu khác nhau, dẫn đến giao diện bị "xé lẻ" (tearing) hoặc bất nhất.

**Solution (useSyncExternalStore):**

```javascript
import { useSyncExternalStore } from 'react';

// Example: Tracking online status from a browser API
function getSnapshot() {
  return navigator.onLine;
}

function subscribe(callback) {
  window.addEventListener('online', callback);
  window.addEventListener('offline', callback);
  return () => {
    window.removeEventListener('online', callback);
    window.removeEventListener('offline', callback);
  };
}

function OnlineStatus() {
  // en: Ensures consistent reading of the store during rendering
  // vi: Đảm bảo việc đọc store nhất quán trong suốt quá trình render
  const isOnline = useSyncExternalStore(subscribe, getSnapshot);
  return <h1>Status: {isOnline ? '🌐 Online' : '❌ Offline'}</h1>;
}
```

---

## 12. Custom Hooks vs HOC (Logic Reuse)
en: **Problem**: How to reuse logic across components? HOCs can lead to "Wrapper Hell" and make props hard to track. Custom Hooks provide a flatter structure but have their own rules.
vi: **Vấn đề**: Làm thế nào để tái sử dụng logic giữa các component? HOC có thể dẫn đến "Wrapper Hell" và khiến việc theo dõi props trở nên khó khăn. Custom Hooks mang lại cấu trúc phẳng hơn nhưng có quy tắc riêng.

**Comparison / So sánh:**

```javascript
// A. HOC Pattern (Legacy/Cluttered)
const withUser = (Component) => (props) => {
  const user = useUser();
  return <Component {...props} user={user} />;
};

// B. Custom Hook Pattern (Modern/Clean)
const useUserLogic = () => {
  const user = useUser();
  return user;
};

// Conclusion / Kết luận:
// en: Use Custom Hooks for most logic. Use HOCs only when you need to inject behavior 
//     WITHOUT modifying the component's internal code (e.g., library integrations).
// vi: Sử dụng Custom Hooks cho hầu hết các trường hợp logic. Chỉ dùng HOC khi bạn cần 
//     chèn hành vi mà KHÔNG muốn sửa mã nội bộ của component (ví dụ: tích hợp thư viện).
```


---

## Summary Checklist / Danh sách kiểm tra tóm tắt
| Category | Priority | Difficulty |
| :--- | :--- | :--- |
| **Foundation** | High | Low |
| **Optimization** | High | Medium |
| **Concurrent React** | High | High |
| **Arch Patterns** | Medium | Medium |
| **Testing/Security** | Medium | High |
