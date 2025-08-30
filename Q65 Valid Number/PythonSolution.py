class Solution:
    def isNumber(self, s: str) -> bool:
        s = s.strip()
        if not s:
            return False

        def is_digits(t: str) -> bool:
            return len(t) > 0 and t.isdigit()

        def is_int(t: str, allow_sign: bool = True) -> bool:
            if allow_sign and t and t[0] in "+-":
                t = t[1:]
            return is_digits(t)

        def is_dec(t: str, allow_sign: bool = True) -> bool:
            if allow_sign and t and t[0] in "+-":
                t = t[1:]
            # must contain exactly one dot
            if t.count(".") != 1:
                return False
            left, right = t.split(".", 1)
            # at least one side must have digits; both sides (if non-empty) must be digits
            if left == "" and right == "":
                return False
            if left and not left.isdigit():
                return False
            if right and not right.isdigit():
                return False
            return True

        # handle exponent
        parts = s.lower().split("e")
        if len(parts) > 2:
            return False

        if len(parts) == 2:
            base, exp = parts[0], parts[1]
            if base == "" or exp == "":
                return False
            base_ok = is_int(base) or is_dec(base)
            exp_ok = is_int(exp)  # exponent must be an integer (optional sign handled inside)
            return base_ok and exp_ok
        else:
            base = parts[0]
            return is_int(base) or is_dec(base)
