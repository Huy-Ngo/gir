gỉ::gỉ! {
    ngoại thùng gỉ;

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

    công_khai(thùng) hàm tuỳ(i: u32) -> Tuỳ<KếtQuả<u32, Xâu>> {
        nếu i % 2 == 1 {
            nếu i == 42 {
                Có(InLỗi(Xâu::từ("đù")))
            } không_thì {
                Có(Được(33))
            }
        } không_thì {
            KhôngCóGì
        }
    }

    bất_đồng_bộ hàm ví_dụ() {
    }

    bất_đồng_bộ hàm ví_dụ_2() {
        ví_dụ().chờ;
    }

    hàm chính() {
        đặt khả_biến x = 31;

        dựa_vào x {
            42 => {
                indòng!("bốn hai")
            }
            _ => indòng!("cái này mới được in nè")
        }

        với i trong 0..10 {
            đặt gt = lặp {
                dừng i;
            };

            khi không_có x < gt {
                x += 1;
            }

            x = nếu đặt Có(kết_quả) = tuỳ(i) {
                kết_quả.tháo_bọc()
            } không_thì {
                12
            };
        }

        //thứ_cấp();
    }

    #[cho_phép(mã_không_được_chạy)]
    hàm thứ_cấp() {
        đù!("ôi không"); // for the true Vietnamese experience
        trời_đất!("lỗi rồi"); // in SFW contexts
        ối!("sai ở đây"); // in SFW contexts, but shorter
    }
}
