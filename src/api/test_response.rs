use maplit::hashmap;
use serde_json;

use crate::api::response::*;

static RSP: &'static str = include_str!("../../tests/BrowserChild.json");

#[test]
fn test_parse() {
    let rsp: Response = serde_json::from_str(RSP).unwrap();

    let expected = Response {
        timedout: false,
        title: "BrowserChild".into(),
        generated: Some(Matches {
            declarations: hashmap! {},
            definitions: hashmap! {},
            files: vec![
                "__GENERATED__/dist/include/nsIBrowserChild.h".into(),
                "__GENERATED__/ipc/ipdl/PBrowserChild.cpp".into(),
                "__GENERATED__/ipc/ipdl/_ipdlheaders/mozilla/dom/PBrowserChild.h".into(),
            ],
            text_matches: hashmap! {},
            uses: hashmap! {
                "BrowserChild".into() => hashmap! {
                    "__GENERATED__/ipc/ipdl/PBrowserChild.cpp".into() => vec![
                        LineMatch {
                            line: "return SendPDocAccessibleConstructor((static_cast<BrowserChild*>(this))->AllocPDocAccessibleChild(aP".into(),
                            number: 418,
                            bounds: (50, 62),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::dom::PBrowserChild::SendPDocAccessibleConstructor".into(),
                                symbol:  "_ZN7mozilla3dom13PBrowserChild29SendPDocAccessibleConstructorEPNS_4a11y19PDocAccessibleChildERKyRKjS8_".into(),
                            }),
                        },
                        LineMatch {
                            line: "return SendPPluginWidgetConstructor((static_cast<BrowserChild*>(this))->AllocPPluginWidgetChild());".into(),
                            number: 476,
                            bounds: (49, 61),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::dom::PBrowserChild::SendPPluginWidgetConstructor".into(),
                                symbol: "_ZN7mozilla3dom13PBrowserChild28SendPPluginWidgetConstructorEv".into(),
                            }),
                        },
                    ],
                },
            },
        }),
        normal: Some(Matches {
            declarations: hashmap! {
                "BrowserChild".into() => hashmap! {
                    "dom/base/TabGroup.h".into() => vec![
                        LineMatch {
                            line: "class BrowserChild;".into(),
                            number: 29,
                            bounds: (6, 18),
                            peek_lines: None,
                            context: None,
                        },
                        LineMatch {
                            line: "class BrowserChild;".into(),
                            number: 47,
                            bounds: (6, 18),
                            peek_lines: None,
                            context: None,
                        },
                    ],

                    "dom/base/nsContentUtils.h".into() => vec![
                        LineMatch {
                            line: "class BrowserChild;".into(),
                            number: 151,
                            bounds: (6, 18),
                            peek_lines: None,
                            context: None,
                        }
                    ],
                },

                "BrowserChildMessageManager".into() => hashmap! {
                    "dom/base/nsWrapperCache.h".into() => vec![
                        LineMatch {
                            line: "class BrowserChildMessageManager;".into(),
                            number: 21,
                            bounds: (6, 18),
                            peek_lines: None,
                            context: None,
                        },
                    ],
                },

                "mozilla::dom::BrowserChild::BrowserChild".into() => hashmap! {
                    "dom/ipc/BrowserChild.h".into() => vec![
                        LineMatch {
                            line: "BrowserChild(ContentChild* aManager, const TabId& aTabId, TabGroup* aTabGroup,".into(),
                            number: 189,
                            bounds: (0, 12),
                            peek_lines: Some("/**\n * Create a new BrowserChild object.\n */\nBrowserChild(ContentChild* aManager, const TabId& aTabId, TabGroup* aTabGroup,\n             const TabContext& aContext, BrowsingContext* aBrowsingContext,\n             uint32_t aChromeFlags, bool aIsTopLevel);\n" .into()),
                            context: Some(MatchContext {
                                context: "mozilla::dom::BrowserChild".into(),
                                symbol: "T_mozilla::dom::BrowserChild".into(),
                            }),
                        },
                    ],
                },
            },
            definitions: hashmap! {
                "BrowserChild".into() => hashmap! {
                    "dom/ipc/BrowserChild.h".into() => vec![
                        LineMatch {
                            line: "class BrowserChild final : public nsMessageManagerScriptExecutor,".into(),
                            number: 149,
                            bounds: (6, 18),
                            peek_lines: Some("/**\n * BrowserChild implements the child actor part of the PBrowser protocol. See\n * PBrowser for more information.\n */\nclass BrowserChild final : public nsMessageManagerScriptExecutor,\n                           public ipc::MessageManagerCallback,\n                           public PBrowserChild,\n                           public nsIWebBrowserChrome2,\n                           public nsIEmbeddingSiteWindow,\n                           public nsIWebBrowserChromeFocus,\n                           public nsIInterfaceRequestor,\n                           public nsIWindowProvider,\n                           public nsSupportsWeakReference,\n                           public nsIBrowserChild,\n                           public nsIObserver,\n                           public nsIWebProgressListener2,\n                           public TabContext,\n                           public nsITooltipListener,\n                           public mozilla::ipc::IShmemAllocator {\n".into()),
                            context: None,
                        },
                    ],
                    "widget/PuppetWidget.h".into() => vec![
                        LineMatch {
                            line: "typedef mozilla::dom::BrowserChild BrowserChild;".into(),
                            number: 46,
                            bounds: (35, 47),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::widget::PuppetWidget".into(),
                                symbol: "T_mozilla::widget::PuppetWidget".into(),
                            }),
                        },
                    ],
                    "widget/nsIWidget.h".into() => vec![
                        LineMatch {
                            line: "typedef mozilla::dom::BrowserChild BrowserChild;".into(),
                            number: 336,
                            bounds: (35, 47),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "nsIWidget".into(),
                                symbol: "T_nsIWidget".into(),
                            }),
                        },
                    ],
                },

                "BrowserChildMap".into() => hashmap! {
                    "dom/ipc/BrowserChild.cpp".into() => vec![
                        LineMatch {
                            line: "typedef nsDataHashtable<nsUint64HashKey, BrowserChild*> BrowserChildMap;".into(),
                            number: 174,
                            bounds: (56, 68),
                            peek_lines: None,
                            context: None,
                        },
                    ],
                },
            },
            files: vec![
                "dom/base/InProcessBrowserChildMessageManager.cpp".into(),
                "dom/base/InProcessBrowserChildMessageManager.h".into(),
                "dom/interfaces/base/nsIBrowserChild.idl".into(),
                "dom/ipc/BrowserChild.cpp".into(),
                "dom/ipc/BrowserChild.h".into(),
            ],
            text_matches: hashmap! {
                "browser/modules/AsyncTabSwitcher.jsm".into() => vec![
                    LineMatch {
                        line: "    // constructing BrowserChild's, layer trees, etc, by showing a blank".into(),
                        number: 360,
                        bounds: (20, 32),
                        peek_lines: None,
                        context: None,
                    },
                ],

                "devtools/server/actors/targets/browsing-context.js".into() => vec![
                    LineMatch {
                        line: "    // BrowserChild, this event is from within this call:".into(),
                        number: 751,
                        bounds: (7, 19),
                        peek_lines: None,
                        context: None,
                    },
                ],
            },
            uses: hashmap! {
                "BrowserChild".into() => hashmap! {
                    "accessible/base/NotificationController.cpp".into() => vec![
                        LineMatch {
                            line: "static_cast<BrowserChild*>(browserChild.get())".into(),
                            number: 926,
                            bounds: (12, 24),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::a11y::NotificationController::WillRefresh".into(),
                                symbol: "_ZN7mozilla4a11y22NotificationController11WillRefreshENS_9TimeStampE,_ZN18nsARefreshObserver11WillRefreshEN7mozilla9TimeStampE".into(),
                            }),
                        },
                    ],
                    "accessible/generic/DocAccessible.cpp".into() => vec![
                        LineMatch {
                            line: "if (RefPtr<dom::BrowserChild> browserChild =".into(),
                            number: 1379,
                            bounds: (16, 28),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::a11y::DocAccessible::DoInitialUpdate".into(),
                                symbol: "_ZN7mozilla4a11y13DocAccessible15DoInitialUpdateEv".into(),
                            }),
                        },
                        LineMatch {
                            line: "dom::BrowserChild::GetFrom(docShell)) {".into(),
                            number: 1380,
                            bounds: (5, 17),
                            peek_lines: None,
                            context: Some(MatchContext {
                                    context: "mozilla::a11y::DocAccessible::DoInitialUpdate".into(),
                                    symbol: "_ZN7mozilla4a11y13DocAccessible15DoInitialUpdateEv".into(),
                            }),
                        }
                    ],
                    "dom/ipc/TabContext.cpp".into() => vec![
                        LineMatch {
                            line: "static_cast<BrowserChild*>(ipcContext.opener().get_PBrowserChild());".into(),
                            number: 153,
                            bounds: (12, 24),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::dom::MaybeInvalidTabContext::MaybeInvalidTabContext".into(),
                                symbol: "_ZN7mozilla3dom22MaybeInvalidTabContextC1ERKNS0_13IPCTabContextE".into(),
                            }),
                        }
                    ],
                },

                "BrowserChildMap".into() => hashmap! {
                    "dom/ipc/BrowserChild.cpp".into() => vec![
                        LineMatch {
                            line: "static BrowserChildMap* sBrowserChildren;".into(),
                            number: 175,
                            bounds: (7, 19),
                            peek_lines: None,
                            context: None,
                        },

                        LineMatch {
                            line: "sBrowserChildren = new BrowserChildMap;".into(),
                            number: 2614,
                            bounds: (23, 35),
                            peek_lines: None,
                            context: Some(MatchContext {
                                context: "mozilla::dom::BrowserChild::InitRenderingState".into(),
                                symbol: "_ZN7mozilla3dom12BrowserChild18InitRenderingStateERKNS_6layers24TextureFactoryIdentifierERKNS2_8LayersIdERKNS2_17CompositorOptionsE".into(),
                            }),
                        },
                    ],
                },
            },
        }),
        test: Some(Matches {
            definitions: hashmap! {},
            declarations: hashmap! {},
            files: vec![],
            text_matches: hashmap! {
                "testing/talos/talos/tests/cpstartup/extension/api.js".into() => vec![
                    LineMatch {
                        line: "const MESSAGES = [\"CPStartup:Go\", \"Content:BrowserChildReady\"];".into(),
                        bounds: (43, 55),
                        number: 30,
                        peek_lines: None,
                        context: None,
                    },
                    LineMatch {
                        line: "      case \"Content:BrowserChildReady\": {".into(),
                        bounds: (20, 32),
                        number: 77,
                        peek_lines: None,
                        context: None,
                    },
                ],
                "toolkit/components/windowwatcher/test/browser_new_content_window_chromeflags.js".into() => vec![
                    LineMatch {
                        line: "        .getInterface(Ci.nsIBrowserChild)".into(),
                        bounds: (28, 40),
                        number: 175,
                        peek_lines: None,
                        context: None,
                    },
                    LineMatch {
                        line: "      // as part of the BrowserChild, so we have to check those too.".into(),
                        bounds: (24, 36),
                        number: 282,
                        peek_lines: None,
                        context: None,
                    },
                ],
            },
            uses: hashmap! {},
        }),
    };

    assert_eq!(rsp, expected);
}
