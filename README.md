\#Wix Build



wix build wix\\main.wxs `

-d Version=1.0.0 `

-d CargoTargetBinDir=target\\release `

-ext WixToolset.UI.wixext `

-o target\\wix\\udtool.msi



