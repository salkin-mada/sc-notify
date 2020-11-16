SCNotify {
	*initClass { | server |
		"\nNotifier present".postln;
	}

	*once {arg message = nil, timeout = nil;
		^super.new.init(message, timeout)
	}

	init { arg msg, time;
		var pwd = Platform.userExtensionDir++"/SCNotify/";
		case
		{ msg.isNil.not and: time.isNil.not } {
			(pwd++"sc-notify -m '%' -t %").format(msg, time).unixCmd;
		}
		{ msg.isNil.not and: time.isNil } {
			(pwd++"sc-notify -m '%'").format(msg).unixCmd;
		}
		{ msg.isNil and: time.isNil.not } {
			(pwd++"sc-notify -t %").format(time).unixCmd;
		}
		{ msg.isNil && time.isNil} {
			(pwd++"sc-notify").unixCmd;

		};
	}
}
