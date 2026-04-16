const $headers = (url?: string) => {
  const h: Record<string, string> = {
    "Content-Type": "application/json",
  };
  const token = window.sessionStorage.getItem("token");
  if (token) {
    const tokenValue = (JSON.parse(token) as Token).token;
    h.authorization = `Bearer ${tokenValue}`;
  }
  if (url) {
    h["X-URL"] = url;
  }
  return h;
};

type ApiResponseError = {
  error: string;
};
type ApiResponse<T> = ApiResponseError | T;

const $fetch = <T>(url: string, options?: RequestInit, body?: any) =>
  fetch(url, {
    ...options,
    headers: $headers(url),
    body: body ? JSON.stringify(body) : undefined,
  })
    .then((res) => res.json() as ApiResponse<T>)
    .then((data) => {
      if ((data as ApiResponseError).error) {
        throw new Error((data as ApiResponseError).error);
      }
      return data;
    });

const $get = <T>(url: string) => $fetch<T>(url);
const $post = <T>(url: string, body: any) =>
  $fetch<T>(url, { method: "POST" }, body);
const $put = <T>(url: string, body: any) =>
  $fetch<T>(url, { method: "PUT" }, body);
const $delete = <T>(url: string) => $fetch<T>(url, { method: "DELETE" });

export { $get, $post, $put, $delete };
