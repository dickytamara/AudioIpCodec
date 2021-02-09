#!/bin/bash 
rm -r -f ./temp
mkdir temp
cd temp
ar -x ../pjproject/pjsip/lib/libpjsua-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjsip/lib/libpjsip-ua-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjsip/lib/libpjsip-simple-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjsip/lib/libpjsip-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjmedia/lib/libpjmedia-codec-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjmedia/lib/libpjmedia-videodev-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjmedia/lib/libpjmedia-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjmedia/lib/libpjmedia-audiodev-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjnath/lib/libpjnath-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjlib-util/lib/libpjlib-util-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/third_party/lib/libsrtp-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/third_party/lib/libresample-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/third_party/lib/libg7221codec-x86_64-unknown-linux-gnu.a
ar -x ../pjproject/pjlib/lib/libpj-x86_64-unknown-linux-gnu.a

ar rv libpjproject.a activesock.o addr_resolv_sock.o aes_gcm_ossl.o aes_icm_ossl.o alaw_ulaw.o alaw_ulaw_table.o \
	alloc.o alsa_dev.o amr_sdp_match.o android_jni_dev.o array.o audio_codecs.o audiodev.o audiotest.o auth.o avi_dev.o \
	avi_player.o base64.o basic_op.o bb10_dev.o bdimad_dev.o bidirectional.o cipher.o cli_console.o cli.o cli_telnet.o \
	clock_thread.o codec.o coef2sam.o colorbar_dev.o common.o conference.o config.o conf_switch.o converter_libswscale.o \
	converter_libyuv.o converter.o crc32.o crypto_kernel.o ctype.o datatypes.o dct4_a.o dct4_s.o decoder.o delaybuf.o \
	dns_dump.o dns.o dns_server.o echo_common.o echo_port.o echo_suppress.o echo_webrtc.o ekt.o encoder.o endpoint.o \
	errno.o event.o evsub_msg.o evsub.o except.o ffmpeg_dev.o ffmpeg_util.o ffmpeg_vid_codecs.o fifobuf.o file_access_unistd.o \
	file_io_ansi.o format.o g711.o g7221.o g7221_sdp_match.o g722_dec.o g722_enc.o g722.o getopt.o guid.o guid_simple.o \
	h263_packetizer.o h264_packetizer.o hash.o hmac_md5.o hmac_ossl.o hmac_sha1.o http_client.o huff_tab.o ice_session.o \
	ice_strans.o ioqueue_select.o ip_helper_generic.o ipp_codecs.o iscomposing.o jbuf.o json.o key.o l16.o legacy_dev.o \
	list.o lock.o log.o log_writer_stdout.o master_port.o md5.o mem_capture.o mem_player.o mwi.o nat_detect.o \
	null_auth.o null_cipher.o null_dev.o null_port.o opencore_amr.o opengl_dev.o openh264.o opensl_dev.o opus.o \
	os_core_unix.o os_error_unix.o os_info.o os_time_common.o os_timestamp_common.o os_timestamp_posix.o os_time_unix.o \
	pa_dev.o passthrough.o pcap.o pidf.o pjsua_acc.o pjsua_aud.o pjsua_call.o pjsua_core.o pjsua_dump.o pjsua_im.o \
	pjsua_media.o pjsua_pres.o pjsua_vid.o plc_common.o pool_buf.o pool_caching.o pool_dbg.o pool.o pool_policy_malloc.o port.o \
	presence_body.o presence.o publishc.o rand.o rbtree.o rdb.o rdbx.o resample_libsamplerate.o resample_port.o \
	resample_resample.o resample_speex.o resamplesubs.o resolver.o rpid.o rtcp_fb.o rtcp.o rtcp_xr.o rtp.o sam2coef.o \
	scanner.o sdl_dev.o sdp_cmp.o sdp_neg.o sdp.o session.o sha1.o silencedet.o silk.o sip_100rel.o sip_auth_aka.o \
	sip_auth_client.o sip_auth_msg.o sip_auth_parser.o sip_auth_server.o sip_config.o sip_dialog.o sip_endpoint.o sip_errno.o \
	sip_inv.o sip_msg.o sip_multipart.o sip_parser.o sip_reg.o sip_replaces.o sip_resolve.o sip_tel_uri.o sip_timer.o \
	sip_transaction.o sip_transport_loop.o sip_transport.o sip_transport_tcp.o sip_transport_tls.o sip_transport_udp.o \
	sip_ua_layer.o sip_uri.o sip_util.o sip_util_proxy.o sip_util_statefull.o sip_xfer.o sock_bsd.o sock_common.o \
	sock_qos_bsd.o sock_qos_common.o sock_select.o sound_legacy.o sound_port.o splitcomb.o srtp_err.o srtp.o srv_resolver.o \
	ssl_sock_common.o ssl_sock_darwin.o ssl_sock_dump.o ssl_sock_gtls.o ssl_sock_ossl.o stat.o stereo_port.o stream_common.o \
	stream_info.o stream.o string.o stun_auth.o stun_msg_dump.o stun_msg.o stun_session.o stun_simple_client.o stun_simple.o \
	stun_sock.o stun_transaction.o tables.o timer.o tonegen.o transport_adapter_sample.o transport_ice.o transport_loop.o \
	transport_srtp.o transport_udp.o turn_session.o turn_sock.o types.o util.o ut_sim.o v4l2_dev.o vid_codec.o vid_codec_util.o \
	vid_conf.o videodev.o vid_port.o vid_stream_info.o vid_stream.o vpx.o wave.o wav_player.o wav_playlist.o wav_writer.o \
	wmme_dev.o wsola.o xml.o xpidf.o

mv libpjproject.a ../
cd ..

#rm -r -f temp
