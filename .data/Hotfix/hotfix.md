## Fix lỗi `error: process didn't exit successfully: target\release\users.exe (exit code: 3)`

https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857

http://mlocati.github.io/articles/gettext-iconv-windows.html

> Tải file: gettext0.21-iconv1.16-shared-64.zip
>
> Giải nén và đổi tên trong thư mục Bin: `libintl-8.dll` → `libintl-9.dll`
>
> Copy `libintl-9.dll` và dán vào thư mục ngang hàng với tệp thực thi .exe (`target\debug` or `target\release`)

```shell
Copy-Item ".\.data\Hotfix\libintl-9.dll" -Destination ".\target\debug"
```