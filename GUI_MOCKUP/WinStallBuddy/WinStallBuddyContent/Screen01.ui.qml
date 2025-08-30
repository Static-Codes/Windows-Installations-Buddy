

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick
import QtQuick.Controls
import WinStallBuddy

Rectangle {
    width: Constants.width
    height: Constants.height
    color: "#000000"
    radius: 1
    border.color: "#d263c7"

    GroupBox {
        id: browserBox
        x: 13
        y: 19
        width: 383
        height: 987
        spacing: -31
        title: qsTr("Browsing")

        Switch {
            id: braveSwitch
            x: 12
            y: 37
            text: qsTr("Brave")
            checkable: true
            display: AbstractButton.TextOnly
        }

        Switch {
            id: chromeSwitch
            x: 12
            y: 78
            text: qsTr("Chrome")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: chromiumSwitch
            x: 12
            y: 119
            text: qsTr("Chromium")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: edgeSwitch
            x: 12
            y: 160
            text: qsTr("Edge")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: firefoxSwitch
            x: 12
            y: 201
            text: qsTr("Firefox")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: librewolfSwitch
            x: 12
            y: 242
            text: qsTr("Librewolf")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: operaSwitch
            x: 12
            y: 283
            text: qsTr("Opera")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: operaGXSwitch
            x: 12
            y: 324
            text: qsTr("Opera GX")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: paleMoonSwitch
            x: 12
            y: 365
            text: qsTr("Pale Moon")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: seamonkeySwitch
            x: 12
            y: 406
            text: qsTr("Seamonkey")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: torSwitch
            x: 12
            y: 447
            text: qsTr("TOR")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: vivaldiSwitch
            x: 12
            y: 488
            text: qsTr("Vivaldi")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        Switch {
            id: waterfoxSwitch
            x: 12
            y: 529
            text: qsTr("Waterfox")
            display: AbstractButton.TextBesideIcon
            checkable: true
        }

        GroupBox {
            id: browserAddonsBox
            x: 12
            y: 580
            width: 339
            height: 395
            title: qsTr("Addons")

            Switch {
                id: customUserChromeSwitch
                x: 0
                y: 37
                text: qsTr("Use Custom userChrome.css (Firefox browsers)")
                enabled: false
                font.pointSize: 9
                icon.height: 16
                display: AbstractButton.TextBesideIcon
                checkable: true
            }
        }
    }

    GroupBox {
        id: gamingBox
        x: 402
        y: 19
        width: 383
        height: 987
        topPadding: 0
        bottomPadding: 12
        spacing: 0
        title: qsTr("Gaming")

        GroupBox {
            id: selectedGamingAppsBox
            x: 5
            y: 121
            width: 347
            height: 854
            rightPadding: 12
            leftPadding: 12
            title: qsTr("Selected:")
            padding: 12

            ListView {
                id: selectedGamingAppsView
                width: parent.width
                height: parent.height
                anchors.fill: parent
                model: selectedAppsModel
                clip: false
                property int selectedIndex: -1 // Stores index for removal

                delegate: Item {
                    width: selectedGamingAppsView.width
                    height: 30

                    Rectangle {
                        width: parent.width
                        height: 30
                        color: "black"
                        border.width: 2
                        border.color: selectedGamingAppsView.selectedIndex
                                      === index ? "#d263c7" : "white" // Sets the item border color to the same color as the app border

                        Text {
                            anchors.centerIn: parent // Centers text within the parent (id=gamingBox)
                            text: model.text
                            color: "white"
                        }

                        MouseArea {
                            anchors.fill: parent
                            onClicked: selectedGamingAppsView.selectedIndex
                                       = index // Store clicked index
                        }
                    }
                }
            }
        }

        ComboBox {
            id: gamingComboBox
            x: 5
            y: 28
            width: 347
            height: 35
            hoverEnabled: true

            model: ListModel {
                ListElement {
                    text: "AMD Adrenalin"
                }
                ListElement {
                    text: "AMD ReLive"
                }
                ListElement {
                    text: "Battle.net Launcher"
                }
                ListElement {
                    text: "Bluestacks Emulator"
                }
                ListElement {
                    text: "CPU Z"
                }
                ListElement {
                    text: "CurseForge"
                }
                ListElement {
                    text: "Epic Games Launcher"
                }
                ListElement {
                    text: "ExitLag"
                }
                ListElement {
                    text: "GPU Z"
                }
                ListElement {
                    text: "HWiNFO"
                }
                ListElement {
                    text: "HWMonitor"
                }
                ListElement {
                    text: "Itch.io Launcher"
                }
                ListElement {
                    text: "Minecraft Launcher
"
                }
                ListElement {
                    text: "MSI Afterburner"
                }
                ListElement {
                    text: "Nexus Manager"
                }
                ListElement {
                    text: "NVDA GeForce Experience"
                }
                ListElement {
                    text: "NVDA ShadowPlay"
                }
                ListElement {
                    text: "OBS Studio"
                }
                ListElement {
                    text: "Parsec"
                }
                ListElement {
                    text: "PingPlotter"
                }
                ListElement {
                    text: "Process Lasso"
                }
                ListElement {
                    text: "Razer Cortex"
                }
                ListElement {
                    text: "Reshade"
                }
                ListElement {
                    text: "Rockstar Launcher"
                }
                ListElement {
                    text: "Roblox Launcher"
                }
                ListElement {
                    text: "Streamlabs OBS"
                }
                ListElement {
                    text: "Steam Launcher"
                }
                ListElement {
                    text: "XPadder"
                }
                ListElement {
                    text: "WTFast"
                }
            }
        }

        Button {
            id: gamingSelectApp
            x: 68
            y: 75
            width: 123
            height: 52
            text: qsTr("Select App")
        }

        ListModel {
            id: selectedAppsModel
        }

        // Since ui.qml files don't support inline javascript, this handles the onclick event callback from a Connections object
        Connections {
            target: gamingSelectApp
            onClicked: {
                let selectedText = gamingComboBox.currentText

                var found = false

                for (var i = 0; i < selectedAppsModel.count; i++) {
                    // start a loop since QML has no operator equivalent to "in" from python
                    if (selectedAppsModel.get(i).text === selectedText) {
                        // if present set the found flag and break
                        found = true
                        break
                    }
                }

                if (found)
                    return

                selectedAppsModel.append({
                                             "text": selectedText
                                         })
            }
        }

        Button {
            id: removeSelectedGamingApp
            x: 196
            y: 75
            width: 157
            height: 52
            text: qsTr("Remove Selected")
        }

        // Another instance of QML being annoying, but I understand why
        Connections {
            target: removeSelectedGamingApp
            function onClicked() {
                if (selectedGamingAppsView.selectedIndex !== -1) {
                    selectedAppsModel.remove(
                                selectedGamingAppsView.selectedIndex)
                    selectedGamingAppsView.selectedIndex = -1 // Resets index after removal
                }
            }
        }
    }

    GroupBox {
        id: programmingBox
        x: 791
        y: 19
        width: 383
        height: 987
        spacing: 0
        topPadding: 0
        bottomPadding: 12
        title: qsTr("Programming")
        ComboBox {
            id: programmingAddonBox
            x: 5
            y: 28
            width: 347
            height: 35
            model: ListModel {
                ListElement {
                    text: "Android Studio"
                }
                ListElement {
                    text: "Azure Data Studio"
                }
                ListElement {
                    text: "Burp Suite"
                }
                ListElement {
                    text: "Docker Desktop"
                }
                ListElement {
                    text: "FileZilla"
                }
                ListElement {
                    text: "Github Desktop"
                }
                ListElement {
                    text: "Eclipse IDE"
                }
                ListElement {
                    text: "Fiddler Classic"
                }
                ListElement {
                    text: "IntelliJ IDEA"
                }
                ListElement {
                    text: "Kubernetes"
                }
                ListElement {
                    text: "MongoDB"
                }
                ListElement {
                    text: "MySQL Workbench"
                }
                ListElement {
                    text: "Nmap"
                }
                ListElement {
                    text: "NodeJS"
                }
                ListElement {
                    text: "Notepad++"
                }
                ListElement {
                    text: "Ollama"
                }
                ListElement {
                    text: "Oracle VirtualBox"
                }
                ListElement {
                    text: "OWASP ZAP"
                }
                ListElement {
                    text: "PostgreSQL"
                }
                ListElement {
                    text: "Postman"
                }
                ListElement {
                    text: "PyCharm"
                }
                ListElement {
                    text: "Python 2.7.18"
                }
                ListElement {
                    text: "Python 3.12"
                }
                ListElement {
                    text: "PuTTY"
                }
                ListElement {
                    text: "Sublime Text"
                }
                ListElement {
                    text: "Visual Studio"
                }
                ListElement {
                    text: "Visual Studio Code"
                }
                ListElement {
                    text: "VMWare Workstation"
                }
                ListElement {
                    text: "WebStorm"
                }
                ListElement {
                    text: "WireShark"
                }
                ListElement {
                    text: "WSL2"
                }
                ListElement {
                    text: "XAMPP"
                }
            }

            hoverEnabled: true
        }

        Button {
            id: programmingSelectApp
            x: 68
            y: 75
            width: 123
            height: 52
            text: qsTr("Select App")
        }

        ListModel {
            id: selectedProgrammingAppsModel
        }

        // Since ui.qml files don't support inline javascript, this handles the onclick event callback from a Connections object
        Connections {
            target: programmingSelectApp
            onClicked: {
                let selectedText = programmingAddonBox.currentText

                var found = false

                for (var i = 0; i < selectedProgrammingAppsModel.count; i++) {
                    // start a loop since QML has no operator equivalent to "in" from python
                    if (selectedProgrammingAppsModel.get(
                                i).text === selectedText) {
                        // if present set the found flag and break
                        found = true
                        break
                    }
                }

                if (found)
                    return

                selectedProgrammingAppsModel.append({
                                                        "text": selectedText
                                                    })
            }
        }

        Button {
            id: removeSelectedProgrammingApp
            x: 196
            y: 75
            width: 157
            height: 52
            text: qsTr("Remove Selected")
        }

        // Another instance of QML being annoying, but I understand why
        Connections {
            target: removeSelectedProgrammingApp
            function onClicked() {
                if (selectedProgrammingAppsView.selectedIndex !== -1) {
                    selectedProgrammingAppsModel.remove(
                                selectedProgrammingAppsView.selectedIndex)
                    selectedProgrammingAppsView.selectedIndex = -1 // Resets index after removal
                }
            }
        }

        GroupBox {
            id: selectedProgrammingAppsBox
            x: 5
            y: 121
            width: 347
            height: 854
            clip: false
            title: qsTr("Selected:")

            ListView {
                id: selectedProgrammingAppsView
                anchors.fill: parent
                model: selectedProgrammingAppsModel
                clip: false
                property int selectedIndex: -1
                x: 0
                y: 0
                width: 347
                height: 854 // Stores index for removal

                delegate: Item {
                    width: selectedProgrammingAppsView.width
                    height: 30

                    Rectangle {
                        width: parent.width
                        height: 30
                        color: "black"
                        border.width: 2
                        border.color: selectedProgrammingAppsView.selectedIndex
                                      === index ? "#d263c7" : "white" // Sets the item border color to the same color as the app border

                        Text {
                            anchors.centerIn: parent // Centers text within the parent (id=gamingBox)
                            text: model.text
                            color: "white"
                        }

                        MouseArea {
                            anchors.fill: parent
                            onClicked: selectedProgrammingAppsView.selectedIndex
                                       = index // Store clicked index
                        }
                    }
                }
            }
        }
    }

    GroupBox {
        id: systemBox
        x: 1180
        y: 19
        width: 383
        height: 987
        font.bold: false
        spacing: 0
        topPadding: 0
        bottomPadding: 12
        title: qsTr("System Utilities")

        ComboBox {
            id: systemAddonBox
            x: 5
            y: 28
            width: 347
            height: 35
            model: ListModel {
                ListElement {
                    name: "7Zip"
                }
                ListElement {
                    name: "Audacious Media Player"
                }
                ListElement {
                    name: "LibreOffice"
                }
                ListElement {
                    name: "Modern CSV"
                }
                ListElement {
                    name: "NoMacs"
                }
                ListElement {
                    name: "Okular"
                }
                ListElement {
                    name: "Photoshop CS6"
                }
                ListElement {
                    name: "VLC Media Player"
                }
                ListElement {
                    name: "WinRAR"
                }
            }
            hoverEnabled: true
        }

        Button {
            id: systemSelectApp
            x: 68
            y: 75
            width: 123
            height: 52
            text: qsTr("Select App")
        }

        Button {
            id: removeSelectedSystemApp
            x: 196
            y: 75
            width: 157
            height: 52
            text: qsTr("Remove Selected")
        }

        ListModel {
            id: selectedSystemAppsModel
        }

        GroupBox {
            id: selectedSystemAppsBox
            x: 5
            y: 121
            width: 347
            height: 854
            title: qsTr("Selected:")
            clip: true

            ListView {
                id: selectedSystemAppsView
                anchors.fill: parent
                model: selectedSystemAppsModel
                clip: false
                property int selectedIndex: -1
                x: -17
                y: -158
                width: 347
                height: 854 // Stores index for removal

                delegate: Item {
                    width: selectedSystemAppsView.width
                    height: 30

                    Rectangle {
                        width: parent.width
                        height: 30
                        color: "black"
                        border.width: 2
                        border.color: selectedSystemAppsView.selectedIndex
                                      === index ? "#d263c7" : "white" // Sets the item border color to the same color as the app border

                        Text {
                            anchors.centerIn: parent // Centers text within the parent (id=gamingBox)
                            text: model.text
                            color: "white"
                        }

                        MouseArea {
                            anchors.fill: parent
                            onClicked: selectedSystemAppsView.selectedIndex
                                       = index // Store clicked index
                        }
                    }
                }
            }
        }

        // Since ui.qml files don't support inline javascript, this handles the onclick event callback from a Connections object
        Connections {
            target: systemSelectApp
            onClicked: {
                let selectedText = systemAddonBox.currentText

                var found = false

                for (var i = 0; i < selectedSystemAppsModel.count; i++) {
                    // start a loop since QML has no operator equivalent to "in" from python
                    if (selectedSystemAppsModel.get(i).text === selectedText) {
                        // if present set the found flag and break
                        found = true
                        break
                    }
                }

                if (found)
                    return

                selectedSystemAppsModel.append({
                                                   "text": selectedText
                                               })
            }
        }

        // Another instance of QML being annoying, but I understand why
        Connections {
            target: removeSelectedSystemApp
            function onClicked() {
                if (selectedSystemAppsView.selectedIndex !== -1) {
                    selectedSystemAppsModel.remove(
                                selectedSystemAppsView.selectedIndex)
                    selectedSystemAppsView.selectedIndex = -1 // Resets index after removal
                }
            }
        }
    }

    Text {
        id: spaceAvailableLabel
        x: 1569
        y: 19
        width: 343
        height: 32
        color: "#ffffff"
        text: qsTr("Space Available: ")
        font.pixelSize: 24
    }

    Text {
        id: spaceRequiredLabel
        x: 1569
        y: 57
        width: 343
        height: 32
        color: "#ffffff"
        text: qsTr("Space Required: ")
        font.pixelSize: 24
    }
}
