<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <!-- For some reason this includes ATSAM4 chip IDs, which we don't want. -->
    <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues">
        <xsl:for-each select="./enumeratedValue">
            <xsl:variable name="evname" select="./name"/>
            <xsl:if test="not(contains($evname, 'SAM4'))">
                <xsl:copy-of select="."/>
            </xsl:if>
        </xsl:for-each>
    </xsl:template>
</xsl:stylesheet>