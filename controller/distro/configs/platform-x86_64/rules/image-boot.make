# -*-makefile-*-
#
# Copyright (C) 2025 by Kat <kat@northernpaws.io>
#
# For further information about the PTXdist project and license conditions
# see the README file.
#

#
# We provide this package
#
IMAGE_PACKAGES-$(PTXCONF_IMAGE_BOOT) += image-boot

#
# Paths and names
#
IMAGE_BOOT		:= image-boot
IMAGE_BOOT_DIR	:= $(BUILDDIR)/$(IMAGE_BOOT)
IMAGE_BOOT_IMAGE	:= $(IMAGEDIR)/boot.img
IMAGE_BOOT_FILES	:= 
IMAGE_BOOT_CONFIG	:= boot.config

# ----------------------------------------------------------------------------
# Image
# ----------------------------------------------------------------------------

IMAGE_BOOT_EFI_STATE_DTS	:= $(PTXDIST_PLATFORMCONFIGDIR)/dts/barebox-state.dts
IMAGE_BOOT_EFI_STATE_DTB	:= $(IMAGEDIR)/barebox-state.dtb

$(IMAGE_BOOT_EFI_STATE_DTB):
	dtc -I dts -O dtb -o $(IMAGE_BOOT_EFI_STATE_DTB) $(IMAGE_BOOT_EFI_STATE_DTS)

$(IMAGE_BOOT_IMAGE): $(IMAGE_BOOT_EFI_STATE_DTB)
	@$(call targetinfo)
	@$(call image/genimage, IMAGE_BOOT)
	@$(call finish)

# vim: syntax=make
