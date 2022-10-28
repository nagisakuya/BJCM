use super::*;

//意味不明ｗ
macro_rules! create_text_and_enum {
    ($_enum:ident,$_fn:ident,$(($_name:ident,[$($_text:expr),*]),)*) => {
        #[derive(PartialEq,Eq,Hash)]
        pub enum $_enum{
            $($_name,)*
        }
        pub(in super::super) fn $_fn() -> HashMap<$_enum,Vec<&'static str>>{
            let mut temp = HashMap::new();
            $(temp.insert($_enum::$_name,vec![ $($_text,)*]);)*
            temp
        }
    }
}

//config.get_text(TextKey::BuyWindowH1)

create_text_and_enum!(TextKey,load_texts,
    (Cancel,["cancel","破棄"]),
    (Apply,["apply","適用"]),
    (BuyWindowButton,["Unlock Full Version","製品版購入"]),
    (BuyWindowName,["About Full Version","製品版について"]),
    (BuyWindowH1,["Difference between Trial Version and Full Version","試用版と製品版の違い"]),
    (BuyWindowT1,[
        "In the Trial Version, table rules cannot be changed.\nIn the Full Version, you can change the table rules and calculate the optimal play and expected value based on the table rules.",
        "試用版では、テーブルルールの変更が出来ません。\n製品版では、テーブルルールを変更し、テーブルルールに対応した最適手と期待値を計算することができます。"
        ]),
    (BuyWindowH2,["How to unlock Full Version","ご購入方法"]),
    (BuyWindowT2,["Please refer to that link","こちらのwebサイトからご購入ください。"]),
    (PurchaseLink,["https://note.com/bjcm/n/nffc8ec41d644","https://note.com/bjcm/n/nffc8ec41d644"]),
    (BuyWindowUserID,["PC-ID:","PC-ID:"]),
    (BuyWindowActivationFormDescription,["Input activation code here.","アクティベーションコードを入力してください。"]),
    (RuleSettingWindowButton,["Rules Setting","ルール設定"]),
    (RuleSettingWindowName,["Rules Setting","ルール設定"]),
    (TrialVersionKeySettingMessage,[
        "In the Trial Version,\nKey settings won't be saved.",
        "試用版ではキー設定を変更しても\n再起動時にリセットされます！"
    ]),
    (KeySettingWindowButton,["Key Config","キー設定"]),
    (KeySettingWindowName,["Key Config","キー設定"]),
    (TrialVersionRuleSettingMessage,[
        "In the Trial Version,\nyou cannot change table rules.",
        "試用版ではルール設定の\n変更ができません！"
    ]),
    (GeneralSettingWindowName,["General","設定"]),
    (GeneralSettingButton,["General","設定"]),
    (GeneralSettingLanguage,["Language","Language"]),
    (GeneralSettingDiscard,["Show Discard area","カードを捨てる場所を表示"]),
    (GeneralSettingRotateNum,["Rotate Number","カードの数字を回転"]),
    (HowToUseButton,["HowToUse","操作説明"]),
    (HowToUseURL,["./data/description/description.html","./data/description/description.html"]),
    (AssetButton,["Asset","資金管理"]),
    (AssetWindowName,["Asset","資金管理"]),
    (AssetWindowAsset,["Total asset","全資金"]),
    (AssetWindowAssetInCasino,["Balance in casino","カジノの残高"]),
    (AssetWindowMinBet,["minimum bet","ミニマムベッド"]),
    (AssetWindowMaxBet,["Maximum bet","マキシマムベッド"]),
    (AssetWindowBetStep,["bet step","ベッド単位"]),
    (AssetWindowThreshold,["round-up threshold","切り上げ閾値"]),
    (Close,["close","閉じる"]),
    (Balance,["balance","残高"]),
    (EstimatedRTPLabel,["TC","TC"]),
    (ActivationButton,["activate","activate"]),
);

    