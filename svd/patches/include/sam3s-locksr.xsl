<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:template match="/device/peripherals/peripheral[starts-with(./name, 'PIO')]/registers">
        <registers>
            <xsl:copy-of select='./register'/>
            <register>
                <name>LOCKSR</name>
                <description>Lock Status</description>
                <addressOffset>0x000000E0</addressOffset>
                <size>32</size>
                <access>read-only</access>
                <resetValue>0x00000000</resetValue>
                <fields>
                    <xsl:call-template name="genLOCKSRFields">
                        <xsl:with-param name="i" select="0"/>
                    </xsl:call-template>
                </fields>
            </register>
        </registers>
    </xsl:template>

    <xsl:template name="genLOCKSRFields">
        <xsl:param name="i"/>
        <xsl:if test="32 > $i">
            <field>
                <name>P<xsl:value-of select="$i"/></name>
                <description>Lock Status.</description>
                <bitOffset><xsl:value-of select="$i"/></bitOffset>
                <bitWidth>1</bitWidth>
                <access>read-only</access>
            </field>
            <xsl:call-template name="genLOCKSRFields">
                <xsl:with-param name="i" select="$i + 1"/>
            </xsl:call-template>
        </xsl:if>
    </xsl:template>
</xsl:stylesheet>
