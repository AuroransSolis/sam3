<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:template match="register">
        <xsl:variable name="dim" select="./dim"/>
        <xsl:choose>
            <xsl:when test="$dim > 0">
                <xsl:call-template name="dimreg"/>
            </xsl:when>
            <xsl:otherwise>
                <xsl:copy-of select="."/>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template name="dimreg">
        <xsl:variable name="rangeS" select="substring-before(./dimIndex, '-')"/>
        <xsl:variable name="rangeE" select="substring-after(./dimIndex, '-')"/>
        <xsl:variable name="name" select="substring-before(./name, '[%s]')"/>
        <xsl:variable name="startAddr">
            <xsl:call-template name="hexToDecimal">
                <xsl:with-param name="hex" select="./addressOffset"/>
            </xsl:call-template>
        </xsl:variable>
        <xsl:variable name="padWidth" select="string-length(substring-after(string(./addressOffset), 'x'))"/>
        <xsl:choose>
            <xsl:when test="$rangeE - $rangeS + 1 = ./dim">
                <xsl:call-template name="dimregrec">
                    <xsl:with-param name="index" select="./dim"/>
                    <xsl:with-param name="rangeS" select="$rangeS"/>
                    <xsl:with-param name="name" select="$name"/>
                    <xsl:with-param name="startAddr" select="$startAddr"/>
                    <xsl:with-param name="padWidth" select="$padWidth"/>
                </xsl:call-template>
            </xsl:when>
            <xsl:otherwise>
                <xsl:message terminate="yes">ERROR: register dimension doesn't match range
                    dimension: <xsl:value-of select="./dim"/>
                    range: <xsl:value-of select="./dimIndex"/>
                </xsl:message>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template name="dimregrec">
        <xsl:param name="index"/>
        <xsl:param name="rangeS"/>
        <xsl:param name="name"/>
        <xsl:param name="startAddr"/>
        <xsl:param name="padWidth"/>
        <xsl:choose>
            <xsl:when test="$index > 0">
                <xsl:variable name="n" select="./dim - $index"/>
                <xsl:variable name="offset" select="$startAddr + $n * ./dimIncrement"/>
                <xsl:variable name="fullName" select="concat($name, $n)"/>
                <xsl:variable name="fullDesc" select="concat(./description, ' ', $n)"/>
                <xsl:variable name="offsetHex">
                    <xsl:call-template name="formatDecimalToHex">
                        <xsl:with-param name="val" select="$offset"/>
                        <xsl:with-param name="width" select="$padWidth"/>
                    </xsl:call-template>
                </xsl:variable>
                <register>
                    <name><xsl:value-of select="$fullName"/></name>
                    <description><xsl:value-of select="$fullDesc"/></description>
                    <addressOffset><xsl:value-of select="$offsetHex"/></addressOffset>
                    <xsl:copy-of select="./*[not(self::dim or self::dimIncrement or self::dimIndex or self::name or self::description or self::addressOffset)]"/>
                </register>
                <xsl:call-template name="dimregrec">
                    <xsl:with-param name="index" select="$index - 1"/>
                    <xsl:with-param name="rangeS" select="$rangeS"/>
                    <xsl:with-param name="name" select="$name"/>
                    <xsl:with-param name="startAddr" select="$startAddr"/>
                    <xsl:with-param name="padWidth" select="$padWidth"/>
                </xsl:call-template>
            </xsl:when>
            <xsl:otherwise/>
        </xsl:choose>
    </xsl:template>

    <xsl:template name="hexToDecimal">
        <xsl:param name="hex"/>
        <xsl:variable name="dec"
            select="string-length(substring-before('0123456789ABCDEF', substring($hex, 1, 1)))"/>
        <xsl:choose>
            <xsl:when test="$hex = ''">0</xsl:when>
            <xsl:otherwise>
                <xsl:variable name="rest">
                    <xsl:call-template name="hexToDecimal">
                        <xsl:with-param name="hex">
                            <xsl:value-of select="substring($hex, 2)"/>
                        </xsl:with-param>
                    </xsl:call-template>
                </xsl:variable>
                <xsl:variable name="powresult">
                    <xsl:call-template name="iPow">
                        <xsl:with-param name="base" select="16"/>
                        <xsl:with-param name="pow" select="string-length($hex) - 1"/>
                    </xsl:call-template>
                </xsl:variable>
                <xsl:value-of select="$dec * $powresult + $rest"/>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template name="iPow">
        <xsl:param name="base"/>
        <xsl:param name="pow"/>
        <xsl:call-template name="iPowRec">
            <xsl:with-param name="base" select="$base"/>
            <xsl:with-param name="pow" select="$pow"/>
            <xsl:with-param name="acc" select="1"/>
        </xsl:call-template>
    </xsl:template>

    <xsl:template name="iPowRec">
        <xsl:param name="base"/>
        <xsl:param name="pow"/>
        <xsl:param name="acc"/>
        <xsl:choose>
            <xsl:when test="$pow > 0">
                <xsl:call-template name="iPowRec">
                    <xsl:with-param name="base" select="$base"/>
                    <xsl:with-param name="pow" select="$pow - 1"/>
                    <xsl:with-param name="acc" select="$acc * $base"/>
                </xsl:call-template>
            </xsl:when>
            <xsl:otherwise>
                <xsl:value-of select="$acc"/>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template name="decimalToHex">
        <xsl:param name="dec"/>
        <xsl:if test="$dec > 0">
            <xsl:call-template name="decimalToHex">
                <xsl:with-param name="dec" select="floor($dec div 16)"/>
            </xsl:call-template>
            <xsl:value-of select="substring('0123456789ABCDEF', (($dec mod 16) + 1), 1)"/>
        </xsl:if>
    </xsl:template>

    <xsl:template name="makeFormatString">
        <xsl:param name="width"/>
        <xsl:call-template name="makeFormatStringRec">
            <xsl:with-param name="width" select="$width"/>
            <xsl:with-param name="string" select="''"/>
        </xsl:call-template>
    </xsl:template>

    <xsl:template name="makeFormatStringRec">
        <xsl:param name="width"/>
        <xsl:param name="string"/>
        <xsl:choose>
            <xsl:when test="string-length($string) &lt; $width">
                <xsl:call-template name="makeFormatStringRec">
                    <xsl:with-param name="width" select="$width"/>
                    <xsl:with-param name="string" select="concat('0', $string)"/>
                </xsl:call-template>
            </xsl:when>
            <xsl:otherwise>
                <xsl:value-of select="$string"/>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template name="formatDecimalToHex">
        <xsl:param name="val"/>
        <xsl:param name="width"/>
        <xsl:variable name="hex">
            <xsl:call-template name="decimalToHex">
                <xsl:with-param name="dec" select="$val"/>
            </xsl:call-template>
        </xsl:variable>
        <xsl:variable name="padding">
            <xsl:call-template name="makeFormatString">
                <xsl:with-param name="width" select="$width"/>
            </xsl:call-template>
        </xsl:variable>
        <xsl:value-of select="concat('0x', substring($padding, 1, $width - string-length(string($hex))), $hex)"/>
    </xsl:template>

    <xsl:template name="main" match="*|@*">
        <xsl:copy>
            <xsl:apply-templates select="node()|@*"/>
        </xsl:copy>
    </xsl:template>
</xsl:stylesheet>
