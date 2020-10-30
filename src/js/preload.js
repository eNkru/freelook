// const {SpellCheckHandler, ContextMenuListener, ContextMenuBuilder} = require('electron-spellchecker');
// Preload.js will fail to load with execption if electron-spellchecker is enabled: 
// https://github.com/electron/electron/issues/18397#issuecomment-583221969
const { ipcRenderer } = require('electron');


function parseEmail(newMail) {

    /* New mail callout:
    <button type="button" class="ms-Button ms-Button--action ms-Button--command _1bZCIOt8pNzUGc5_4EecwV root-118"
        data-storybook="newmail" data-is-focusable="true">
        <span class="ms-Button-flexContainer _2ffM28EUzuZTnw7XOLJhyM flexContainer-86"
            data-automationid="splitbuttonprimary">
            <div role="presentation"
                class="ms-Persona-coin ms-Persona--size32 _2wocbClJzEBB0NRX1fR2LX _3fxSw_KFta16t28JP70e_s coin-95">
                <div role="presentation" class="ms-Persona-imageArea imageArea-119">
                    <div class="ms-Persona-initials initials-122" aria-hidden="true">
                        <span>SD</span>
                    </div>
                </div>
            </div>
            <div class="_9Ie6sVh0VX-ro9NAPwFKb">
                <div class="_6tY-cxX-513mYdh6yJLFJ">EMAIL ADDRESS</div>
                <div class="_3D_rMrrL9OQqN4yRDBU0wB">EMAIL SUBJECT</div>
                <div class="_3EYcW96jXMSxYrgKGACvQ5">
                        EMAIL TEXT
                </div>
            </div>
        </span>
    </button>
    <button type="button" class="ms-Button ms-Button--icon _1jj-F7glQvCHYDjDwBM16j root-58" data-is-focusable="true">
        <span class="ms-Button-flexContainer flexContainer-45" data-automationid="splitbuttonprimary">
            <i data-icon-name="Cancel" aria-hidden="true" class="ms-Button-icon _2NRERAuLG4KyyZOTDhVoH0 icon-60">
            </i>
        </span>
    </button>
    */
    if (
        // Button has one child element (Wrapper)
        newMail.childNodes.length === 1 &&
        // Element below button has 2 child element (Persona coin and email data wrapper)
        newMail.childNodes[0].childNodes.length === 2 &&
        // Element 1 has 3 childs (Email address, email subject, email text)
        newMail.childNodes[0].childNodes[1].childNodes.length === 3
    ) {
        const emailWrapper = newMail.childNodes[0].childNodes[1];
        ipcRenderer.send('showNotification', {
            type: "email",
            data: {
                address: emailWrapper.childNodes[0].textContent,
                subject: emailWrapper.childNodes[1].textContent
            }
        });
    } else {
        console.log("Could not read new mail callout since expected hierarchy not fulfilled", addedNode.outerHTML);
    }

}

function parseReminder(reminder) {

    /* New reminder callout
    <button type="button" data-storybook="reminder"
        class="ms-Button _3rsUP82WSAWrd2_ltrkYqu ms-Button--action ms-Button--command root-115" data-is-focusable="true"
        tabindex="-1">
        <span class="ms-Button-flexContainer _1_WrKcTbnYdDAFqzC4SSeW flexContainer-86"
            data-automationid="splitbuttonprimary">
            <div class="_1No_MTwEqWqrxpLMLyewVV" id="CharmControlID57"></div>
            <div class="_2FExrHciwQ0OfU3n4wXbm4">
                <div class="_3JvUSBrE7i4EvsSuQ4qQaX">
                    <div class="PW6IONUfEQd0sJHkwWKio">Test</div>
                    <div class="_2LTiFXMdIPUAYBsZ924g6x">Now</div>
                </div>
                <div class="_152OXH0Wv5AJy2MYNeNvBF">
                    <div class="_3lanHQZYgbRG-wj0O20psN">11:30</div>
                    <div class="Pqix-KFjAnL4CfIddz4Kw"></div>
                </div>
            </div>
        </span>
    </button>
    */
    if (
        // Button has one child element (Wrapper)
        reminder.childNodes.length === 1 &&
        // Element below button has 2 child element (div and reminder data wrapper)
        reminder.childNodes[0].childNodes.length === 2 &&
        // Element 1 has 2 childs (Reminder text and time)
        reminder.childNodes[0].childNodes[1].childNodes.length === 2
    ) {
        const reminderWrapper = reminder.childNodes[0].childNodes[1];
        ipcRenderer.send('showNotification', {
            type: "reminder",
            data: {
                text: reminderWrapper.childNodes[0].childNodes[0].textContent,
                time: reminderWrapper.childNodes[1].childNodes[0].textContent
            }
        });
    } else {
        console.log("Could not read new reminder callout since expected hierarchy not fulfilled", addedNode.outerHTML);
    }

}

function registerCalloutObserver() {
    console.log("Trying to register callout observer");
    const layerDiv = document.querySelector(".ms-Layer-content");
    if (layerDiv) {
        const observer = new MutationObserver((mutations) => {
            try {
                for (const mutation of mutations) {
                    for (const addedNode of mutation.addedNodes) {
                        const newMail = addedNode.querySelector("button[data-storybook='newmail']");
                        if (newMail) {
                            parseEmail(newMail);
                        } else {

                            const reminder = addedNode.querySelector("button[data-storybook='reminder']");
                            if (reminder) {
                                parseReminder(reminder);
                            } else {
                                console.log(addedNode.outerHTML);
                            }
                        }
                    }
                }
            } catch (ex) {
                console.log(ex);
            }
        });

        console.log("Starting observer for notifications");
        observer.observe(layerDiv, { childList: true, subtree: true });

        console.log("Searching for existing reminders");
        const reminders = document.querySelectorAll("button[data-storybook='reminder']");
        for (const reminder of reminders) {
            parseReminder(reminder);
        }


    } else {
        console.log("Could not register notification observer. Notifications not supported, Retrying in 5 seconds");
        setTimeout(() => {
            registerCalloutObserver();
        }, 5000);
    }

}

ipcRenderer.on('registerCalloutObserver', function (event, data) {
    setTimeout(() => {
        registerCalloutObserver();
    }, 5000);
});

// Preload.js will fail to load with execption if electron-spellchecker is enabled: 
// https://github.com/electron/electron/issues/18397#issuecomment-583221969
/*setTimeout(() => {
    window.spellCheckHandler = new SpellCheckHandler();
    window.spellCheckHandler.attachToInput();
    window.spellCheckHandler.switchLanguage('en-US');

    const contextMenuBuilder = new ContextMenuBuilder(window.spellCheckHandler);
    const contextMenuListener = new ContextMenuListener((info) => {
        contextMenuBuilder.showPopupMenu(info);
    });
}, 5000);*/