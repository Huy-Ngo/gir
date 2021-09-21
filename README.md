# gỉ

![Gỉ's logo](https://github.com/Huy-Ngo/gir/blob/ch%C3%ADnh/logo.png?raw=true)

Aren't you *mệt mỏi* from writing Rust programs in English?
Do you like saying "đù" a lot?
Would you like to try something different, in an
exotic and funny-sounding language?
Would you want to bring some Vietnamese touch to your programs?

**gỉ** (Vietnamese for _Rust_) is here to save your day, as it allows you to
write Rust programs in Vietnamese, using Vietnamese keywords, Vietnamese function names,
Vietnamese idioms.

You're not from Vietnam mainland and don't feel at ease using only Vietnamese
words? Don't worry!  Vietnamese Rust is fully compatible with English-Rust, so
you can mix both at your convenience.

Here's an example of what can be achieved with Gỉ:

### trait and impl (aka tính chất et triển khai)

```rust
gỉ::gỉ! {
    dùng chuẩn::collections::ÁnhXạBăm như ÁnhXạ;

    tính_chất KhoáGiáTrị {
        hàm viết(&bản_thân, khoá: Xâu, giá_trị: Xâu);
        hàm đọc(&bản_thân, giá_trị: Xâu) -> Tuỳ<&Xâu>;
    }

    tĩnh khả_biến TỪ_ĐIỂN: Tuỳ<ÁnhXạ<Xâu, Chaine>> = Rỗng;

    cấu_trúc CụThể;

    triển_khai KhoáGiáTrị với CụThể {
        hàm viết(&bản_thân, khoá: Xâu, giá_trị: Xâu) {
            đặt tđ = nguy_hiểm {
                TỪ_ĐIỂN.đọc_hoặc_chèn_với(MặcĐịnh::mặc_định)
            };
            tđ.chèn(khoá, giá_trị);
        }
        hàm đọc(&bản_thân, khoá: Xâu) -> KếtQuả<Tuỳ<&Xâu>, Xâu> {
            nếu đặt Có(tđ) = nguy_hiểm { TỪ_ĐIỂN.bằng_tham_chiếu() } {
                Được(tđ.đọc(&khoá))
            } không_thì {
                InLỗi("đang lấy tđ (từ điển)".vào())
            }
        }
    }
}
```

### Support for various ideolects

```rust
#[cho_phép(mã_không_được_chạy)]
hàm thứ_cấp() {
    đù!("ôi không"); // for the true Vietnamese experience
    trời_đất!("lỗi rồi"); // in SFW contexts
    ối!("sai ở đây"); // in SFW contexts, but shorter
}
```

Additionally, you can put tone diacritics the old way (`Tùy`) or the current
standard way (`Tuỳ`)

### Other examples

See the [examples](./ví_dụ/src/main.rs) to get a rough sense of the whole
syntax.

## đóng góp

First of all, _cám ơn nhiều_ for considering participating to this joke, the
Vietnamese government will thank you later! Feel free to throw in a few
identifiers here and there, and open a pull-request against the `chính`
(Vietnamese for `main`) branch.

## but why would you do zat

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- [others][others] can do it so why not

## Other languages

See the [original shitpost][others]
for a more up-to-date list.

[others]: https://github.com/bnjbvr/rouille#other-languages

## Lời cám ơn

- [@dinhanhx](https://github.com/dinhanhx) for making a logo!

## Giấy phép

[Giấy phép Công cộng Thích làm gì thì làm](https://github.com/Huy-Ngo/gỉ/blob/chính/GIẤY_PHÉP),
the official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.
