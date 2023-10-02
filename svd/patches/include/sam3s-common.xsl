<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <!-- For some reason this includes a bunch chip IDs we don't want. -->
    <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues">
        <enumeratedValues>
            <xsl:for-each select="./enumeratedValue">
                <xsl:variable name="evname" select="./name"/>
                <xsl:if test="contains($evname, 'SAM3S')">
                    <xsl:copy-of select="."/>
                </xsl:if>
            </xsl:for-each>
        </enumeratedValues>
    </xsl:template>
</xsl:stylesheet>
